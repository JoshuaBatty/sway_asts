[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 9,
                    end: 17,
                    as_str(): "dep lib;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 13,
                    end: 16,
                    as_str(): "lib",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0de482d80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
            ),
            start: 9,
            end: 17,
            as_str(): "dep lib;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 22,
                            end: 23,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 26,
                            end: 27,
                            as_str(): "B",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0de482d80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
            ),
            start: 18,
            end: 36,
            as_str(): "use A::{B, C, D,};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 22,
                            end: 23,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 29,
                            end: 30,
                            as_str(): "C",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0de482d80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
            ),
            start: 18,
            end: 36,
            as_str(): "use A::{B, C, D,};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 22,
                            end: 23,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 32,
                            end: 33,
                            as_str(): "D",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0de482d80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
            ),
            start: 18,
            end: 36,
            as_str(): "use A::{B, C, D,};",
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
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 41,
                            end: 45,
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
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 65,
                                                    end: 66,
                                                    as_str(): "x",
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
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 69,
                                                                        end: 70,
                                                                        as_str(): "B",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 69,
                                                                end: 70,
                                                                as_str(): "B",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 82,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 85,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 85,
                                                                    as_str(): "b: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 69,
                                                    end: 92,
                                                    as_str(): "B {\n        b: 0,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 61,
                                    end: 93,
                                    as_str(): "let x = B {\n        b: 0,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 103,
                                                    as_str(): "y",
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
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "C",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 107,
                                                                as_str(): "C",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 118,
                                                                        end: 119,
                                                                        as_str(): "c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 121,
                                                                        end: 122,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 118,
                                                                    end: 122,
                                                                    as_str(): "c: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 106,
                                                    end: 129,
                                                    as_str(): "C {\n        c: 0,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 130,
                                    as_str(): "let y = C {\n        c: 0,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 140,
                                                    as_str(): "z",
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
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 143,
                                                                        end: 144,
                                                                        as_str(): "D",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 143,
                                                                end: 144,
                                                                as_str(): "D",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 155,
                                                                        end: 156,
                                                                        as_str(): "d",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 158,
                                                                        end: 159,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 155,
                                                                    end: 159,
                                                                    as_str(): "d: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 166,
                                                    as_str(): "D {\n        d: 0,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 167,
                                    as_str(): "let z = D {\n        d: 0,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 176,
                                                    end: 179,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromTrait {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 192,
                                                                                end: 193,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 192,
                                                                                end: 193,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "add",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0de482d80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 193,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 192,
                                                                end: 193,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: MethodApplication(
                                                                    MethodApplicationExpression {
                                                                        method_name_binding: TypeBinding {
                                                                            inner: FromTrait {
                                                                                call_path: CallPath {
                                                                                    prefixes: [
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "core",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                ),
                                                                                                start: 186,
                                                                                                end: 187,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                ),
                                                                                                start: 186,
                                                                                                end: 187,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "add",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0de482d80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                            ),
                                                                                            start: 186,
                                                                                            end: 187,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 186,
                                                                                end: 187,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Subfield(
                                                                                    SubfieldExpression {
                                                                                        prefix: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0de482d80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                        ),
                                                                                                        start: 182,
                                                                                                        end: 183,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                ),
                                                                                                start: 182,
                                                                                                end: 183,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                ),
                                                                                                start: 184,
                                                                                                end: 185,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 182,
                                                                                    end: 185,
                                                                                    as_str(): "x.b",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Subfield(
                                                                                    SubfieldExpression {
                                                                                        prefix: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0de482d80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                        ),
                                                                                                        start: 188,
                                                                                                        end: 189,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                ),
                                                                                                start: 188,
                                                                                                end: 189,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                                ),
                                                                                                start: 190,
                                                                                                end: 191,
                                                                                                as_str(): "c",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 188,
                                                                                    end: 191,
                                                                                    as_str(): "y.c",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 182,
                                                                    end: 191,
                                                                    as_str(): "x.b + y.c",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Subfield(
                                                                    SubfieldExpression {
                                                                        prefix: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0de482d80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 195,
                                                                                        as_str(): "z",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 194,
                                                                                end: 195,
                                                                                as_str(): "z",
                                                                            },
                                                                        },
                                                                        field_to_access: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 196,
                                                                                end: 197,
                                                                                as_str(): "d",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 197,
                                                                    as_str(): "z.d",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 197,
                                                    as_str(): "x.b + y.c + z.d",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 172,
                                    end: 198,
                                    as_str(): "let foo = x.b + y.c + z.d;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 206,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0de482d80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                            ),
                                            start: 203,
                                            end: 206,
                                            as_str(): "foo",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 203,
                                    end: 206,
                                    as_str(): "foo",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 55,
                            end: 208,
                            as_str(): "{\n    let x = B {\n        b: 0,\n    };\n    let y = C {\n        c: 0,\n    };\n    let z = D {\n        d: 0,\n    };\n    let foo = x.b + y.c + z.d;\n    foo\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0de482d80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                        ),
                        start: 38,
                        end: 208,
                        as_str(): "fn main() -> u64 {\n    let x = B {\n        b: 0,\n    };\n    let y = C {\n        c: 0,\n    };\n    let z = D {\n        d: 0,\n    };\n    let foo = x.b + y.c + z.d;\n    foo\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0de482d80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                        ),
                        start: 51,
                        end: 54,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0de482d80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
            ),
            start: 38,
            end: 208,
            as_str(): "fn main() -> u64 {\n    let x = B {\n        b: 0,\n    };\n    let y = C {\n        c: 0,\n    };\n    let z = D {\n        d: 0,\n    };\n    let foo = x.b + y.c + z.d;\n    foo\n}",
        },
    },
]
