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
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 12,
                            end: 20,
                            as_str(): "identity",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 39,
                                                    end: 40,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 39,
                                            end: 40,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 39,
                                    end: 40,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 35,
                            end: 42,
                            as_str(): "{\n  x\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 24,
                                    end: 25,
                                    as_str(): "x",
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 28,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 27,
                                end: 28,
                                as_str(): "T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 9,
                        end: 42,
                        as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 33,
                                end: 34,
                                as_str(): "T",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        T: TypeId(7249),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 33,
                        end: 34,
                        as_str(): "T",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 9,
            end: 42,
            as_str(): "fn identity<T>(x: T) -> T {\n  x\n}",
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
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 47,
                            end: 59,
                            as_str(): "two_generics",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 88,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 87,
                                            end: 88,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 87,
                                    end: 88,
                                    as_str(): "b",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 83,
                            end: 90,
                            as_str(): "{\n  b\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 67,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 70,
                                        as_str(): "A",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 69,
                                end: 70,
                                as_str(): "A",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 72,
                                    end: 73,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 76,
                                        as_str(): "B",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 75,
                                end: 76,
                                as_str(): "B",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 44,
                        end: 90,
                        as_str(): "fn two_generics<A, B>(a: A, b: B) -> B {\n  b\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 81,
                                end: 82,
                                as_str(): "B",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        A: TypeId(7250),
                        B: TypeId(7251),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 81,
                        end: 82,
                        as_str(): "B",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 44,
            end: 90,
            as_str(): "fn two_generics<A, B>(a: A, b: B) -> B {\n  b\n}",
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
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 95,
                            end: 109,
                            as_str(): "three_generics",
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 150,
                                                    end: 151,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 153,
                                                        end: 154,
                                                        as_str(): "A",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 153,
                                                    end: 154,
                                                    as_str(): "A",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc097840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 158,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 158,
                                                    as_str(): "a",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 146,
                                    end: 159,
                                    as_str(): "let a: A = a;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 162,
                                                    end: 163,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 162,
                                            end: 163,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 163,
                                    as_str(): "b",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 142,
                            end: 165,
                            as_str(): "{\n  let a: A = a;\n  b\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 119,
                                    end: 120,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 122,
                                        end: 123,
                                        as_str(): "A",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 122,
                                end: 123,
                                as_str(): "A",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 125,
                                    end: 126,
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 128,
                                        end: 129,
                                        as_str(): "B",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 128,
                                end: 129,
                                as_str(): "B",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 131,
                                    end: 132,
                                    as_str(): "c",
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
                                        src (ptr): 0x00007fe0fc097840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                        ),
                                        start: 134,
                                        end: 135,
                                        as_str(): "C",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 134,
                                end: 135,
                                as_str(): "C",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 92,
                        end: 165,
                        as_str(): "fn three_generics<A, B, C>(a: A, b: B, c: C) -> B {\n  let a: A = a;\n  b\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc097840,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                ),
                                start: 140,
                                end: 141,
                                as_str(): "B",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        A: TypeId(7252),
                        B: TypeId(7253),
                        C: TypeId(7254),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 140,
                        end: 141,
                        as_str(): "B",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 92,
            end: 165,
            as_str(): "fn three_generics<A, B, C>(a: A, b: B, c: C) -> B {\n  let a: A = a;\n  b\n}",
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
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 170,
                            end: 174,
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
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 194,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Boolean,
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 196,
                                                    end: 200,
                                                    as_str(): "bool",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 213,
                                                                        as_str(): "identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 205,
                                                                end: 213,
                                                                as_str(): "identity",
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
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 214,
                                                                    end: 218,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 219,
                                                    as_str(): "identity(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 189,
                                    end: 220,
                                    as_str(): "let a: bool   = identity(true);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 227,
                                                    end: 228,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 230,
                                                    end: 233,
                                                    as_str(): "u32",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 239,
                                                                        end: 247,
                                                                        as_str(): "identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 239,
                                                                end: 247,
                                                                as_str(): "identity",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 248,
                                                                    end: 250,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 239,
                                                    end: 251,
                                                    as_str(): "identity(10)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 223,
                                    end: 252,
                                    as_str(): "let b: u32    = identity(10);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 259,
                                                    end: 260,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 262,
                                                    end: 265,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 271,
                                                                        end: 279,
                                                                        as_str(): "identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 271,
                                                                end: 279,
                                                                as_str(): "identity",
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
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 280,
                                                                    end: 282,
                                                                    as_str(): "42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 271,
                                                    end: 283,
                                                    as_str(): "identity(42)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 255,
                                    end: 284,
                                    as_str(): "let c: u64    = identity(42);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 291,
                                                    end: 292,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Str(
                                                Length {
                                                    val: 3,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 299,
                                                        as_str(): "3",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 294,
                                                    end: 300,
                                                    as_str(): "str[3]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 311,
                                                                        as_str(): "identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 303,
                                                                end: 311,
                                                                as_str(): "identity",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    String(
                                                                        Span {
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 313,
                                                                            end: 316,
                                                                            as_str(): "foo",
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 312,
                                                                    end: 317,
                                                                    as_str(): "\"foo\"",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 303,
                                                    end: 318,
                                                    as_str(): "identity(\"foo\")",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 287,
                                    end: 319,
                                    as_str(): "let e: str[3] = identity(\"foo\");",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 328,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 330,
                                                    end: 333,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 336,
                                                                        end: 348,
                                                                        as_str(): "two_generics",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 336,
                                                                end: 348,
                                                                as_str(): "two_generics",
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
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 349,
                                                                    end: 353,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 355,
                                                                    end: 357,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 336,
                                                    end: 358,
                                                    as_str(): "two_generics(true, 10)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 323,
                                    end: 359,
                                    as_str(): "let f: u64 = two_generics(true, 10);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 366,
                                                    end: 367,
                                                    as_str(): "g",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Str(
                                                Length {
                                                    val: 3,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc097840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                        ),
                                                        start: 373,
                                                        end: 374,
                                                        as_str(): "3",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 369,
                                                    end: 375,
                                                    as_str(): "str[3]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc097840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 378,
                                                                        end: 392,
                                                                        as_str(): "three_generics",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc097840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                ),
                                                                start: 378,
                                                                end: 392,
                                                                as_str(): "three_generics",
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
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 393,
                                                                    end: 397,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    String(
                                                                        Span {
                                                                            src (ptr): 0x00007fe0fc097840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 400,
                                                                            end: 403,
                                                                            as_str(): "foo",
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 399,
                                                                    end: 404,
                                                                    as_str(): "\"foo\"",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc097840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 406,
                                                                    end: 408,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 378,
                                                    end: 409,
                                                    as_str(): "three_generics(true, \"foo\", 10)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 362,
                                    end: 410,
                                    as_str(): "let g: str[3] = three_generics(true, \"foo\", 10);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc097840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                                    ),
                                                    start: 414,
                                                    end: 415,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc097840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                            ),
                                            start: 414,
                                            end: 415,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc097840,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                                    ),
                                    start: 414,
                                    end: 415,
                                    as_str(): "a",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc097840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                            ),
                            start: 185,
                            end: 418,
                            as_str(): "{\n  let a: bool   = identity(true);\n  let b: u32    = identity(10);\n  let c: u64    = identity(42);\n  let e: str[3] = identity(\"foo\");\n\n  let f: u64 = two_generics(true, 10);\n  let g: str[3] = three_generics(true, \"foo\", 10);\n\n  a\n\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 167,
                        end: 418,
                        as_str(): "fn main() -> bool {\n  let a: bool   = identity(true);\n  let b: u32    = identity(10);\n  let c: u64    = identity(42);\n  let e: str[3] = identity(\"foo\");\n\n  let f: u64 = two_generics(true, 10);\n  let g: str[3] = three_generics(true, \"foo\", 10);\n\n  a\n\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc097840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
                        ),
                        start: 180,
                        end: 184,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc097840,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBdZg9F/generic_functions/src/main.sw",
            ),
            start: 167,
            end: 418,
            as_str(): "fn main() -> bool {\n  let a: bool   = identity(true);\n  let b: u32    = identity(10);\n  let c: u64    = identity(42);\n  let e: str[3] = identity(\"foo\");\n\n  let f: u64 = two_generics(true, 10);\n  let g: str[3] = three_generics(true, \"foo\", 10);\n\n  a\n\n}",
        },
    },
]
