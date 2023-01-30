[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 16,
                            end: 30,
                            as_str(): "DoubleIdentity",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 46,
                                    as_str(): "first",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 49,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 41,
                                end: 49,
                                as_str(): "first: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 48,
                                end: 49,
                                as_str(): "T",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 53,
                                    end: 59,
                                    as_str(): "second",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 62,
                                        as_str(): "F",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 53,
                                end: 62,
                                as_str(): "second: F",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 61,
                                end: 62,
                                as_str(): "F",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7249),
                        F: TypeId(7250),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0f8c2c380,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                        ),
                        start: 9,
                        end: 64,
                        as_str(): "struct DoubleIdentity<T, F> {\n  first: T,\n  second: F\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f8c2c380,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
            ),
            start: 9,
            end: 64,
            as_str(): "struct DoubleIdentity<T, F> {\n  first: T,\n  second: F\n}",
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
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 69,
                            end: 84,
                            as_str(): "double_identity",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Struct(
                                            StructExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 145,
                                                                as_str(): "DoubleIdentity",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 131,
                                                        end: 145,
                                                        as_str(): "DoubleIdentity",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 152,
                                                                end: 157,
                                                                as_str(): "first",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 159,
                                                                        end: 160,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 159,
                                                                end: 160,
                                                                as_str(): "x",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 152,
                                                            end: 160,
                                                            as_str(): "first: x",
                                                        },
                                                    },
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 172,
                                                                as_str(): "second",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 174,
                                                                        end: 175,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 174,
                                                                end: 175,
                                                                as_str(): "y",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f8c2c380,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 175,
                                                            as_str(): "second: y",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 131,
                                            end: 179,
                                            as_str(): "DoubleIdentity {\n    first: x,\n    second: y\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 131,
                                    end: 179,
                                    as_str(): "DoubleIdentity {\n    first: x,\n    second: y\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 127,
                            end: 181,
                            as_str(): "{\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 92,
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
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 94,
                                        end: 95,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 94,
                                end: 95,
                                as_str(): "T",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 98,
                                    as_str(): "y",
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
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 101,
                                        as_str(): "F",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 100,
                                end: 101,
                                as_str(): "F",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0f8c2c380,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                        ),
                        start: 66,
                        end: 181,
                        as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0f8c2c380,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                ),
                                start: 106,
                                end: 120,
                                as_str(): "DoubleIdentity",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7251,
                                    ),
                                    initial_type_id: TypeId(
                                        7251,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 121,
                                        end: 122,
                                        as_str(): "T",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        7252,
                                    ),
                                    initial_type_id: TypeId(
                                        7252,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0f8c2c380,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                        ),
                                        start: 124,
                                        end: 125,
                                        as_str(): "F",
                                    },
                                },
                            ],
                        ),
                    },
                    type_parameters: [
                        T: TypeId(7253),
                        F: TypeId(7254),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0f8c2c380,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                        ),
                        start: 106,
                        end: 126,
                        as_str(): "DoubleIdentity<T, F>",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f8c2c380,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
            ),
            start: 66,
            end: 181,
            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity {\n    first: x,\n    second: y\n  }\n}",
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
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 186,
                            end: 190,
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
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 209,
                                                    end: 217,
                                                    as_str(): "double_a",
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 235,
                                                                        as_str(): "double_identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 220,
                                                                end: 235,
                                                                as_str(): "double_identity",
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
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 236,
                                                                    end: 240,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 242,
                                                                    end: 246,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 220,
                                                    end: 247,
                                                    as_str(): "double_identity(true, true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 205,
                                    end: 248,
                                    as_str(): "let double_a = double_identity(true, true);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 255,
                                                    end: 263,
                                                    as_str(): "double_b",
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 266,
                                                                        end: 281,
                                                                        as_str(): "double_identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 266,
                                                                end: 281,
                                                                as_str(): "double_identity",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U32(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 282,
                                                                    end: 287,
                                                                    as_str(): "10u32",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        43,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 289,
                                                                    end: 294,
                                                                    as_str(): "43u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 266,
                                                    end: 295,
                                                    as_str(): "double_identity(10u32, 43u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 251,
                                    end: 296,
                                    as_str(): "let double_b = double_identity(10u32, 43u64);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 333,
                                                    end: 341,
                                                    as_str(): "double_a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 343,
                                                        end: 357,
                                                        as_str(): "DoubleIdentity",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                71,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                71,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 358,
                                                                end: 362,
                                                                as_str(): "bool",
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 364,
                                                                end: 368,
                                                                as_str(): "bool",
                                                            },
                                                        },
                                                    ],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 343,
                                                    end: 369,
                                                    as_str(): "DoubleIdentity<bool, bool>",
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 372,
                                                                        end: 387,
                                                                        as_str(): "double_identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 372,
                                                                end: 387,
                                                                as_str(): "double_identity",
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
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 388,
                                                                    end: 392,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 394,
                                                                    end: 398,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 372,
                                                    end: 399,
                                                    as_str(): "double_identity(true, true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 329,
                                    end: 400,
                                    as_str(): "let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 407,
                                                    end: 415,
                                                    as_str(): "double_b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 417,
                                                        end: 431,
                                                        as_str(): "DoubleIdentity",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                33,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                33,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 432,
                                                                end: 435,
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
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 437,
                                                                end: 440,
                                                                as_str(): "u64",
                                                            },
                                                        },
                                                    ],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 417,
                                                    end: 441,
                                                    as_str(): "DoubleIdentity<u32, u64>",
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
                                                                        src (ptr): 0x00007fe0f8c2c380,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                        ),
                                                                        start: 444,
                                                                        end: 459,
                                                                        as_str(): "double_identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 444,
                                                                end: 459,
                                                                as_str(): "double_identity",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U32(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 460,
                                                                    end: 465,
                                                                    as_str(): "10u32",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        43,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f8c2c380,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                    ),
                                                                    start: 467,
                                                                    end: 472,
                                                                    as_str(): "43u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f8c2c380,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                    ),
                                                    start: 444,
                                                    end: 473,
                                                    as_str(): "double_identity(10u32, 43u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 403,
                                    end: 474,
                                    as_str(): "let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Subfield(
                                            SubfieldExpression {
                                                prefix: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f8c2c380,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                                ),
                                                                start: 478,
                                                                end: 486,
                                                                as_str(): "double_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 478,
                                                        end: 486,
                                                        as_str(): "double_a",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f8c2c380,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                                        ),
                                                        start: 487,
                                                        end: 492,
                                                        as_str(): "first",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f8c2c380,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                            ),
                                            start: 478,
                                            end: 492,
                                            as_str(): "double_a.first",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f8c2c380,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                                    ),
                                    start: 478,
                                    end: 492,
                                    as_str(): "double_a.first",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0f8c2c380,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                            ),
                            start: 201,
                            end: 494,
                            as_str(): "{\n  let double_a = double_identity(true, true);\n  let double_b = double_identity(10u32, 43u64);\n\n  // for testing annotations\n  let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);\n\n  double_a.first\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0f8c2c380,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                        ),
                        start: 183,
                        end: 494,
                        as_str(): "fn main() -> bool {\n  let double_a = double_identity(true, true);\n  let double_b = double_identity(10u32, 43u64);\n\n  // for testing annotations\n  let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);\n\n  double_a.first\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0f8c2c380,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
                        ),
                        start: 196,
                        end: 200,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f8c2c380,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW3UvTM/generic_structs/src/main.sw",
            ),
            start: 183,
            end: 494,
            as_str(): "fn main() -> bool {\n  let double_a = double_identity(true, true);\n  let double_b = double_identity(10u32, 43u64);\n\n  // for testing annotations\n  let double_a: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let double_b: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);\n\n  double_a.first\n}",
        },
    },
]
