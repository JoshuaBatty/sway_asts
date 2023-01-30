[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007f8a18734b80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 42,
                            end: 43,
                            as_str(): "S",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 50,
                                    end: 51,
                                    as_str(): "t",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [
                                    TypeArgument {
                                        type_id: TypeId(
                                            21,
                                        ),
                                        initial_type_id: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 57,
                                            as_str(): "u64",
                                        },
                                    },
                                ],
                            ),
                            span: Span {
                                src (ptr): 0x00007f8a18734b80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                ),
                                start: 50,
                                end: 59,
                                as_str(): "t: (u64,)",
                            },
                            type_span: Span {
                                src (ptr): 0x00007f8a18734b80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                ),
                                start: 53,
                                end: 59,
                                as_str(): "(u64,)",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007f8a18734b80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                        ),
                        start: 35,
                        end: 61,
                        as_str(): "struct S {\n    t: (u64,)\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a18734b80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
            ),
            start: 35,
            end: 61,
            as_str(): "struct S {\n    t: (u64,)\n}",
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
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 91,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 94,
                                                                        end: 95,
                                                                        as_str(): "S",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 95,
                                                                as_str(): "S",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "t",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Tuple(
                                                                        [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        2,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 110,
                                                                                    end: 111,
                                                                                    as_str(): "2",
                                                                                },
                                                                            },
                                                                        ],
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 109,
                                                                        end: 113,
                                                                        as_str(): "(2,)",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a18734b80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                    ),
                                                                    start: 106,
                                                                    end: 113,
                                                                    as_str(): "t: (2,)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 119,
                                                    as_str(): "S {\n        t: (2,)\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 120,
                                    as_str(): "let a = S {\n        t: (2,)\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 130,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: CodeBlock(
                                                    CodeBlock {
                                                        contents: [
                                                            AstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        VariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_1",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 139,
                                                                                    end: 140,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 139,
                                                                                            end: 140,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 139,
                                                                                    end: 140,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a18734b80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                    ),
                                                                    start: 133,
                                                                    end: 170,
                                                                    as_str(): "match a {\n        S { t } => t,\n    }",
                                                                },
                                                            },
                                                            AstNode {
                                                                content: ImplicitReturnExpression(
                                                                    Expression {
                                                                        kind: Match(
                                                                            MatchExpression {
                                                                                value: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_1",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                ),
                                                                                                start: 139,
                                                                                                end: 140,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 139,
                                                                                        end: 140,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                    ),
                                                                                                    start: 151,
                                                                                                    end: 152,
                                                                                                    as_str(): "S",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                            ),
                                                                                                            start: 155,
                                                                                                            end: 156,
                                                                                                            as_str(): "t",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                        ),
                                                                                                        start: 155,
                                                                                                        end: 156,
                                                                                                        as_str(): "t",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                ),
                                                                                                start: 151,
                                                                                                end: 158,
                                                                                                as_str(): "S { t }",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                        ),
                                                                                                        start: 162,
                                                                                                        end: 163,
                                                                                                        as_str(): "t",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                ),
                                                                                                start: 162,
                                                                                                end: 163,
                                                                                                as_str(): "t",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 151,
                                                                                            end: 164,
                                                                                            as_str(): "S { t } => t,",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 133,
                                                                            end: 170,
                                                                            as_str(): "match a {\n        S { t } => t,\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a18734b80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                    ),
                                                                    start: 133,
                                                                    end: 170,
                                                                    as_str(): "match a {\n        S { t } => t,\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 133,
                                                            end: 170,
                                                            as_str(): "match a {\n        S { t } => t,\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 170,
                                                    as_str(): "match a {\n        S { t } => t,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 125,
                                    end: 171,
                                    as_str(): "let b = match a {\n        S { t } => t,\n    };",
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
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 182,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a18734b80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                        ),
                                                        start: 176,
                                                        end: 182,
                                                        as_str(): "assert",
                                                    },
                                                },
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
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 187,
                                                                                        end: 189,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 187,
                                                                                        end: 189,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 187,
                                                                                    end: 189,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 189,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                ),
                                                                                                start: 183,
                                                                                                end: 184,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 183,
                                                                                        end: 184,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                },
                                                                                index: 0,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 185,
                                                                                    end: 186,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 183,
                                                                            end: 186,
                                                                            as_str(): "b.0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 190,
                                                                            end: 191,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 183,
                                                            end: 191,
                                                            as_str(): "b.0 == 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 176,
                                            end: 192,
                                            as_str(): "assert(b.0 == 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 192,
                                    as_str(): "assert(b.0 == 2)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 199,
                                            end: 200,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a18734b80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                    ),
                                    start: 199,
                                    end: 200,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 80,
                            end: 202,
                            as_str(): "{\n    let a = S {\n        t: (2,)\n    };\n    let b = match a {\n        S { t } => t,\n    };\n    assert(b.0 == 2);\n\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a18734b80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                        ),
                        start: 63,
                        end: 202,
                        as_str(): "fn main() -> u64 {\n    let a = S {\n        t: (2,)\n    };\n    let b = match a {\n        S { t } => t,\n    };\n    assert(b.0 == 2);\n\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a18734b80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                        ),
                        start: 76,
                        end: 79,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a18734b80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
            ),
            start: 63,
            end: 202,
            as_str(): "fn main() -> u64 {\n    let a = S {\n        t: (2,)\n    };\n    let b = match a {\n        S { t } => t,\n    };\n    assert(b.0 == 2);\n\n    1\n}",
        },
    },
]
