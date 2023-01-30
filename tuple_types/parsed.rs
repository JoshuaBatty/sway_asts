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
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 12,
                            end: 24,
                            as_str(): "gimme_a_unit",
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
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 37,
                                                    end: 38,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Tuple(
                                                [],
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 42,
                                                    as_str(): "()",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Tuple(
                                                    [],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 47,
                                                    as_str(): "()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 33,
                                    end: 48,
                                    as_str(): "let x: () = ();",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 53,
                                                    end: 54,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 53,
                                            end: 54,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 53,
                                    end: 54,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 27,
                            end: 56,
                            as_str(): "{\n    let x: () = ();\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 9,
                        end: 56,
                        as_str(): "fn gimme_a_unit() {\n    let x: () = ();\n    x\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 9,
                        end: 26,
                        as_str(): "fn gimme_a_unit()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 9,
            end: 56,
            as_str(): "fn gimme_a_unit() {\n    let x: () = ();\n    x\n}",
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
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 61,
                            end: 78,
                            as_str(): "also_gimme_a_unit",
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
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 98,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Tuple(
                                                [],
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 100,
                                                    end: 102,
                                                    as_str(): "()",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Tuple(
                                                    [],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 107,
                                                    as_str(): "()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 93,
                                    end: 108,
                                    as_str(): "let x: () = ();",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 114,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 113,
                                            end: 114,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 113,
                                    end: 114,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 87,
                            end: 116,
                            as_str(): "{\n    let x: () = ();\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 58,
                        end: 116,
                        as_str(): "fn also_gimme_a_unit() -> () {\n    let x: () = ();\n    x\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 84,
                        end: 86,
                        as_str(): "()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 58,
            end: 116,
            as_str(): "fn also_gimme_a_unit() -> () {\n    let x: () = ();\n    x\n}",
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
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 121,
                            end: 141,
                            as_str(): "gimme_a_single_value",
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
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 164,
                                                    end: 165,
                                                    as_str(): "x",
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
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 168,
                                                            end: 171,
                                                            as_str(): "u32",
                                                        },
                                                    },
                                                ],
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 167,
                                                    end: 173,
                                                    as_str(): "(u32,)",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Literal(
                                                                U32(
                                                                    123,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 177,
                                                                end: 183,
                                                                as_str(): "123u32",
                                                            },
                                                        },
                                                    ],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 176,
                                                    end: 185,
                                                    as_str(): "(123u32,)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 160,
                                    end: 186,
                                    as_str(): "let x: (u32,) = (123u32,);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 191,
                                                    end: 192,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 191,
                                            end: 192,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 191,
                                    end: 192,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 154,
                            end: 194,
                            as_str(): "{\n    let x: (u32,) = (123u32,);\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 118,
                        end: 194,
                        as_str(): "fn gimme_a_single_value() -> (u32,) {\n    let x: (u32,) = (123u32,);\n    x\n}",
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
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 148,
                                    end: 151,
                                    as_str(): "u32",
                                },
                            },
                        ],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 147,
                        end: 153,
                        as_str(): "(u32,)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 118,
            end: 194,
            as_str(): "fn gimme_a_single_value() -> (u32,) {\n    let x: (u32,) = (123u32,);\n    x\n}",
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
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 199,
                            end: 211,
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
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 235,
                                                        end: 239,
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
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 241,
                                                        end: 245,
                                                        as_str(): "2u64",
                                                    },
                                                },
                                            ],
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 234,
                                            end: 246,
                                            as_str(): "(1u32, 2u64)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 234,
                                    end: 246,
                                    as_str(): "(1u32, 2u64)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 228,
                            end: 248,
                            as_str(): "{\n    (1u32, 2u64)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 196,
                        end: 248,
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
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 218,
                                    end: 221,
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
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 223,
                                    end: 226,
                                    as_str(): "u64",
                                },
                            },
                        ],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 217,
                        end: 227,
                        as_str(): "(u32, u64)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 196,
            end: 248,
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
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 253,
                            end: 257,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 273,
                                                                end: 285,
                                                                as_str(): "gimme_a_unit",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 273,
                                                        end: 285,
                                                        as_str(): "gimme_a_unit",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 273,
                                            end: 287,
                                            as_str(): "gimme_a_unit()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 273,
                                    end: 287,
                                    as_str(): "gimme_a_unit()",
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
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 293,
                                                                end: 310,
                                                                as_str(): "also_gimme_a_unit",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 293,
                                                        end: 310,
                                                        as_str(): "also_gimme_a_unit",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 293,
                                            end: 312,
                                            as_str(): "also_gimme_a_unit()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 293,
                                    end: 312,
                                    as_str(): "also_gimme_a_unit()",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 322,
                                                    end: 323,
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
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 326,
                                                                        end: 346,
                                                                        as_str(): "gimme_a_single_value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 326,
                                                                end: 346,
                                                                as_str(): "gimme_a_single_value",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 326,
                                                    end: 348,
                                                    as_str(): "gimme_a_single_value()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 318,
                                    end: 349,
                                    as_str(): "let x = gimme_a_single_value();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 358,
                                                    end: 359,
                                                    as_str(): "b",
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
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 362,
                                                                        end: 374,
                                                                        as_str(): "gimme_a_pair",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 362,
                                                                end: 374,
                                                                as_str(): "gimme_a_pair",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 362,
                                                    end: 376,
                                                    as_str(): "gimme_a_pair()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 354,
                                    end: 377,
                                    as_str(): "let b = gimme_a_pair();",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                123,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 382,
                                            end: 385,
                                            as_str(): "123",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28725630,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                    ),
                                    start: 382,
                                    end: 385,
                                    as_str(): "123",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 267,
                            end: 387,
                            as_str(): "{\n    gimme_a_unit();\n    also_gimme_a_unit();\n    let x = gimme_a_single_value();\n    let b = gimme_a_pair();\n    123\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 250,
                        end: 387,
                        as_str(): "fn main() -> u32 {\n    gimme_a_unit();\n    also_gimme_a_unit();\n    let x = gimme_a_single_value();\n    let b = gimme_a_pair();\n    123\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                        ),
                        start: 263,
                        end: 266,
                        as_str(): "u32",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28725630,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
            ),
            start: 250,
            end: 387,
            as_str(): "fn main() -> u32 {\n    gimme_a_unit();\n    also_gimme_a_unit();\n    let x = gimme_a_single_value();\n    let b = gimme_a_pair();\n    123\n}",
        },
    },
]
