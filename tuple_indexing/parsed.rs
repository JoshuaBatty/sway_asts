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
                                                        src (ptr): 0x00007f8a20305dd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a20305dd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                            ),
                            start: 41,
                            end: 61,
                            as_str(): "{\n    (1u32, 2u64)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a20305dd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                                    src (ptr): 0x00007f8a20305dd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                                    src (ptr): 0x00007f8a20305dd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                        src (ptr): 0x00007f8a20305dd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                        ),
                        start: 30,
                        end: 40,
                        as_str(): "(u32, u64)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20305dd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
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
                                                        arguments: [],
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: TupleIndex(
                                            TupleIndexExpression {
                                                prefix: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20305dd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                                ),
                                                                start: 114,
                                                                end: 115,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
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
                                                index: 0,
                                                index_span: Span {
                                                    src (ptr): 0x00007f8a20305dd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 117,
                                                    as_str(): "0",
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
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a20305dd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                            ),
                            start: 80,
                            end: 119,
                            as_str(): "{\n    let x = gimme_a_pair();\n    x.0\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a20305dd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
                        ),
                        start: 63,
                        end: 119,
                        as_str(): "fn main() -> u32 {\n    let x = gimme_a_pair();\n    x.0\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a20305dd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZ0WGHC/tuple_indexing/src/main.sw",
            ),
            start: 63,
            end: 119,
            as_str(): "fn main() -> u32 {\n    let x = gimme_a_pair();\n    x.0\n}",
        },
    },
]
