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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Tuple(
                                            [
                                                Expression {
                                                    kind: Literal(
                                                        U32(
                                                            1,
                                                        ),
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
                                                Expression {
                                                    kind: Literal(
                                                        U64(
                                                            2,
                                                        ),
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 41,
                            end: 61,
                            as_str(): "{\n    (1u32, 2u64)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe032b8a4f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                        ),
                        start: 9,
                        end: 61,
                        as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
                    },
                    return_type: Tuple(
                        [
                            TypeArgument {
                                type_id: TypeId(
                                    33,
                                ),
                                initial_type_id: TypeId(
                                    33,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 31,
                                    end: 34,
                                    as_str(): "u32",
                                },
                            },
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe032b8a4f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 39,
                                    as_str(): "u64",
                                },
                            },
                        ],
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe032b8a4f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
            ),
            start: 9,
            end: 61,
            as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Tuple(
                                                [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            7249,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7249,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
                                                            as_str(): "T",
                                                        },
                                                    },
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            7250,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7250,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 112,
                                                            as_str(): "E",
                                                        },
                                                    },
                                                ],
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
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 118,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
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
                                                        Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 120,
                                                                        end: 121,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 108,
                                                        end: 109,
                                                        as_str(): "T",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 95,
                                                            end: 123,
                                                            as_str(): "let (x, y): (T, E) = (a, b);",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
                                                        as_str(): "E",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 95,
                                                            end: 123,
                                                            as_str(): "let (x, y): (T, E) = (a, b);",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 89,
                            end: 125,
                            as_str(): "{\n    let (x, y): (T, E) = (a, b);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 81,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                        FunctionParameter {
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 86,
                                        end: 87,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                    span: Span {
                        src (ptr): 0x00007fe032b8a4f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                        ),
                        start: 63,
                        end: 125,
                        as_str(): "fn test<T, E>(a: T, b: E) {\n    let (x, y): (T, E) = (a, b);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [
                        T: TypeId(7251),
                        E: TypeId(7252),
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe032b8a4f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
            ),
            start: 63,
            end: 125,
            as_str(): "fn test<T, E>(a: T, b: E) {\n    let (x, y): (T, E) = (a, b);\n}",
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
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
                                                        arguments: [],
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 151,
                                                            end: 183,
                                                            as_str(): "let (foo, bar) = gimme_a_pair();",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 151,
                                                            end: 183,
                                                            as_str(): "let (foo, bar) = gimme_a_pair();",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Tuple(
                                                [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            33,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 204,
                                                            as_str(): "u32",
                                                        },
                                                    },
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            71,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 210,
                                                            as_str(): "bool",
                                                        },
                                                    },
                                                ],
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
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    10,
                                                                ),
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
                                                        Expression {
                                                            kind: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 188,
                                                            end: 225,
                                                            as_str(): "let (x, y): (u32, bool) = (10, true);",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Boolean,
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 188,
                                                            end: 225,
                                                            as_str(): "let (x, y): (u32, bool) = (10, true);",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
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
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                false,
                                                            ),
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
                                                ],
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
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                42,
                                                            ),
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
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                42,
                                                            ),
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
                                                ],
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Tuple(
                                                [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            21,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 372,
                                                            end: 375,
                                                            as_str(): "u64",
                                                        },
                                                    },
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            7255,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7255,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 377,
                                                            end: 399,
                                                            as_str(): "(u32, (bool, str[2]) )",
                                                        },
                                                    },
                                                ],
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
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Literal(
                                                                U64(
                                                                    42,
                                                                ),
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
                                                        Expression {
                                                            kind: Tuple(
                                                                [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                42,
                                                                            ),
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
                                                                    Expression {
                                                                        kind: Tuple(
                                                                            [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
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
                                                                                Expression {
                                                                                    kind: Literal(
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 437,
                                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Tuple(
                                                [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            33,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 381,
                                                            as_str(): "u32",
                                                        },
                                                    },
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            7256,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7256,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 383,
                                                            end: 397,
                                                            as_str(): "(bool, str[2])",
                                                        },
                                                    },
                                                ],
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 437,
                                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 437,
                                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Tuple(
                                                [
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            71,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 384,
                                                            end: 388,
                                                            as_str(): "bool",
                                                        },
                                                    },
                                                    TypeArgument {
                                                        type_id: TypeId(
                                                            7253,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            7253,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 396,
                                                            as_str(): "str[2]",
                                                        },
                                                    },
                                                ],
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 437,
                                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Boolean,
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 437,
                                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Str(
                                                Length {
                                                    val: 2,
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 394,
                                                        end: 395,
                                                        as_str(): "2",
                                                    },
                                                },
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
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
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
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 437,
                                                            as_str(): "let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );",
                                                        },
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 442,
                                                    end: 443,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 145,
                            end: 445,
                            as_str(): "{\n    let (foo, bar) = gimme_a_pair();\n    let (x, y): (u32, bool) = (10, true);\n    //let (x, y): (u32, _) = (42, true); // this generates a parsing error\n    test(true, false);\n    test (42, 42);\n    let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );\n    a\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe032b8a4f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                        ),
                        start: 128,
                        end: 445,
                        as_str(): "fn main() -> u32 {\n    let (foo, bar) = gimme_a_pair();\n    let (x, y): (u32, bool) = (10, true);\n    //let (x, y): (u32, _) = (42, true); // this generates a parsing error\n    test(true, false);\n    test (42, 42);\n    let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );\n    a\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe032b8a4f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
            ),
            start: 128,
            end: 445,
            as_str(): "fn main() -> u32 {\n    let (foo, bar) = gimme_a_pair();\n    let (x, y): (u32, bool) = (10, true);\n    //let (x, y): (u32, _) = (42, true); // this generates a parsing error\n    test(true, false);\n    test (42, 42);\n    let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );\n    a\n}",
        },
    },
]
