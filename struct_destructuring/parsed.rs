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
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 43,
                            end: 57,
                            as_str(): "gimme_a_struct",
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
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 75,
                                                                end: 80,
                                                                as_str(): "Dummy",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 75,
                                                        end: 80,
                                                        as_str(): "Dummy",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 83,
                                                                end: 89,
                                                                as_str(): "value1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    1,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 92,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 83,
                                                            end: 92,
                                                            as_str(): "value1: 1",
                                                        },
                                                    },
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 100,
                                                                as_str(): "value2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 106,
                                                                as_str(): "true",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 106,
                                                            as_str(): "value2: true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 108,
                                            as_str(): "Dummy { value1: 1, value2: true }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 75,
                                    end: 108,
                                    as_str(): "Dummy { value1: 1, value2: true }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 69,
                            end: 110,
                            as_str(): "{\n    Dummy { value1: 1, value2: true }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                        ),
                        start: 40,
                        end: 110,
                        as_str(): "fn gimme_a_struct() -> Dummy {\n    Dummy { value1: 1, value2: true }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 63,
                                end: 68,
                                as_str(): "Dummy",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                        ),
                        start: 63,
                        end: 68,
                        as_str(): "Dummy",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 40,
            end: 110,
            as_str(): "fn gimme_a_struct() -> Dummy {\n    Dummy { value1: 1, value2: true }\n}",
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
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 115,
                            end: 119,
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
                                                    "__destructure_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 144,
                                                    as_str(): "Dummy",
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
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 166,
                                                                        end: 180,
                                                                        as_str(): "gimme_a_struct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 180,
                                                                as_str(): "gimme_a_struct",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 182,
                                                    as_str(): "gimme_a_struct()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 183,
                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 147,
                                                    end: 153,
                                                    as_str(): "value1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__destructure_1",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 144,
                                                                        as_str(): "Dummy",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 183,
                                                                as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 147,
                                                                end: 153,
                                                                as_str(): "value1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 135,
                                                    end: 183,
                                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 183,
                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 155,
                                                    end: 161,
                                                    as_str(): "value2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__destructure_1",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 144,
                                                                        as_str(): "Dummy",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 183,
                                                                as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 155,
                                                                end: 161,
                                                                as_str(): "value2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 135,
                                                    end: 183,
                                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 183,
                                    as_str(): "let Dummy { value1, value2 } = gimme_a_struct();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 197,
                                                    as_str(): "Dummy",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 218,
                                                        end: 223,
                                                        as_str(): "Dummy",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 218,
                                                    end: 223,
                                                    as_str(): "Dummy",
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
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 226,
                                                                        end: 240,
                                                                        as_str(): "gimme_a_struct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 226,
                                                                end: 240,
                                                                as_str(): "gimme_a_struct",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 226,
                                                    end: 242,
                                                    as_str(): "gimme_a_struct()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 243,
                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 200,
                                                    end: 206,
                                                    as_str(): "value1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__destructure_2",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 197,
                                                                        as_str(): "Dummy",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 188,
                                                                end: 243,
                                                                as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 200,
                                                                end: 206,
                                                                as_str(): "value1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 243,
                                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 243,
                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 208,
                                                    end: 214,
                                                    as_str(): "value2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__destructure_2",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 197,
                                                                        as_str(): "Dummy",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 188,
                                                                end: 243,
                                                                as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 214,
                                                                as_str(): "value2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 243,
                                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 243,
                                    as_str(): "let Dummy { value1, value2 }: Dummy = gimme_a_struct();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 252,
                                                    end: 256,
                                                    as_str(): "data",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 259,
                                                                        end: 263,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 259,
                                                                end: 263,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 274,
                                                                        end: 279,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 281,
                                                                        end: 283,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b383550,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 274,
                                                                    end: 283,
                                                                    as_str(): "value: 42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 259,
                                                    end: 290,
                                                    as_str(): "Data {\n        value: 42,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 248,
                                    end: 291,
                                    as_str(): "let data = Data {\n        value: 42,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_3",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 300,
                                                    end: 304,
                                                    as_str(): "Data",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b383550,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 316,
                                                        end: 320,
                                                        as_str(): "Data",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 316,
                                                    end: 320,
                                                    as_str(): "Data",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 327,
                                                            as_str(): "data",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 323,
                                                    end: 327,
                                                    as_str(): "data",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 296,
                                    end: 328,
                                    as_str(): "let Data { value }: Data = data;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 307,
                                                    end: 312,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__destructure_3",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b383550,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 300,
                                                                        end: 304,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 296,
                                                                end: 328,
                                                                as_str(): "let Data { value }: Data = data;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b383550,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 307,
                                                                end: 312,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 296,
                                                    end: 328,
                                                    as_str(): "let Data { value }: Data = data;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 296,
                                    end: 328,
                                    as_str(): "let Data { value }: Data = data;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b383550,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 345,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04b383550,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 340,
                                                    end: 345,
                                                    as_str(): "value",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04b383550,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                            ),
                                            start: 333,
                                            end: 345,
                                            as_str(): "return value",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 333,
                                    end: 345,
                                    as_str(): "return value",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 129,
                            end: 348,
                            as_str(): "{\n    let Dummy { value1, value2 } = gimme_a_struct();\n    let Dummy { value1, value2 }: Dummy = gimme_a_struct();\n    let data = Data {\n        value: 42,\n    };\n    let Data { value }: Data = data;\n    return value;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                        ),
                        start: 112,
                        end: 348,
                        as_str(): "fn main() -> u64 {\n    let Dummy { value1, value2 } = gimme_a_struct();\n    let Dummy { value1, value2 }: Dummy = gimme_a_struct();\n    let data = Data {\n        value: 42,\n    };\n    let Data { value }: Data = data;\n    return value;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                        ),
                        start: 125,
                        end: 128,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 112,
            end: 348,
            as_str(): "fn main() -> u64 {\n    let Dummy { value1, value2 } = gimme_a_struct();\n    let Dummy { value1, value2 }: Dummy = gimme_a_struct();\n    let data = Data {\n        value: 42,\n    };\n    let Data { value }: Data = data;\n    return value;\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 357,
                            end: 361,
                            as_str(): "Data",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 369,
                                    end: 374,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 369,
                                end: 379,
                                as_str(): "value: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 376,
                                end: 379,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                        ),
                        start: 350,
                        end: 382,
                        as_str(): "struct Data { \n    value: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 350,
            end: 382,
            as_str(): "struct Data { \n    value: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04b383550,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                            ),
                            start: 391,
                            end: 396,
                            as_str(): "Dummy",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 403,
                                    end: 409,
                                    as_str(): "value1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 403,
                                end: 414,
                                as_str(): "value1: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 411,
                                end: 414,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04b383550,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                    ),
                                    start: 420,
                                    end: 426,
                                    as_str(): "value2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 420,
                                end: 432,
                                as_str(): "value2: bool",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04b383550,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                                ),
                                start: 428,
                                end: 432,
                                as_str(): "bool",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe04b383550,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
                        ),
                        start: 384,
                        end: 435,
                        as_str(): "struct Dummy {\n    value1: u64,\n    value2: bool,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04b383550,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKidE5d/struct_destructuring/src/main.sw",
            ),
            start: 384,
            end: 435,
            as_str(): "struct Dummy {\n    value1: u64,\n    value2: bool,\n}",
        },
    },
]
