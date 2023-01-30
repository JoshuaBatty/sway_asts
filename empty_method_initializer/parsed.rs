[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
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
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 24,
                            end: 28,
                            as_str(): "b512",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 30,
                            end: 34,
                            as_str(): "B512",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb0f57bed10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
            ),
            start: 9,
            end: 75,
            as_str(): "use std::{\n    b512::B512,\n    assert::assert,\n    logging::log\n};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
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
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 40,
                            end: 46,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 48,
                            end: 54,
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
            src (ptr): 0x00007fb0f57bed10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
            ),
            start: 9,
            end: 75,
            as_str(): "use std::{\n    b512::B512,\n    assert::assert,\n    logging::log\n};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
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
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 60,
                            end: 67,
                            as_str(): "logging",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 69,
                            end: 72,
                            as_str(): "log",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb0f57bed10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
            ),
            start: 9,
            end: 75,
            as_str(): "use std::{\n    b512::B512,\n    assert::assert,\n    logging::log\n};",
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
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 80,
                            end: 84,
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
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 112,
                                                    as_str(): "hi_bits",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: B256,
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 114,
                                                    end: 118,
                                                    as_str(): "b256",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    B256(
                                                        [
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                            119,
                                                        ],
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 121,
                                                    end: 187,
                                                    as_str(): "0x7777777777777777777777777777777777777777777777777777777777777777",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 101,
                                    end: 188,
                                    as_str(): "let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 197,
                                                    end: 204,
                                                    as_str(): "lo_bits",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: B256,
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 206,
                                                    end: 210,
                                                    as_str(): "b256",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    B256(
                                                        [
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                            0,
                                                        ],
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 213,
                                                    end: 279,
                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 193,
                                    end: 280,
                                    as_str(): "let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 290,
                                                    end: 291,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 294,
                                                                                end: 298,
                                                                                as_str(): "B512",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 294,
                                                                            end: 298,
                                                                            as_str(): "B512",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 300,
                                                                            end: 304,
                                                                            as_str(): "from",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 294,
                                                                end: 304,
                                                                as_str(): "B512::from",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Tuple(
                                                                    [
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                        ),
                                                                                        start: 306,
                                                                                        end: 313,
                                                                                        as_str(): "hi_bits",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 306,
                                                                                end: 313,
                                                                                as_str(): "hi_bits",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                        ),
                                                                                        start: 315,
                                                                                        end: 322,
                                                                                        as_str(): "lo_bits",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 315,
                                                                                end: 322,
                                                                                as_str(): "lo_bits",
                                                                            },
                                                                        },
                                                                    ],
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 305,
                                                                    end: 323,
                                                                    as_str(): "(hi_bits, lo_bits)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 294,
                                                    end: 324,
                                                    as_str(): "B512::from((hi_bits, lo_bits))",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 286,
                                    end: 325,
                                    as_str(): "let b = B512::from((hi_bits, lo_bits));",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 334,
                                                    end: 341,
                                                    as_str(): "other_b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 344,
                                                                                end: 348,
                                                                                as_str(): "B512",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 348,
                                                                            as_str(): "B512",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 350,
                                                                            end: 353,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 344,
                                                                end: 353,
                                                                as_str(): "B512::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 344,
                                                    end: 355,
                                                    as_str(): "B512::new()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 330,
                                    end: 356,
                                    as_str(): "let other_b = B512::new();",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: LazyOperator(
                                            LazyOperatorExpression {
                                                op: And,
                                                lhs: Expression {
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
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 375,
                                                                                    end: 377,
                                                                                    as_str(): "!=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 375,
                                                                                    end: 377,
                                                                                    as_str(): "!=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "neq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 375,
                                                                                end: 377,
                                                                                as_str(): "!=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 375,
                                                                    end: 377,
                                                                    as_str(): "!=",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: ArrayIndex(
                                                                        ArrayIndexExpression {
                                                                            prefix: Expression {
                                                                                kind: Subfield(
                                                                                    SubfieldExpression {
                                                                                        prefix: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                        ),
                                                                                                        start: 363,
                                                                                                        end: 364,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 363,
                                                                                                end: 364,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 365,
                                                                                                end: 370,
                                                                                                as_str(): "bytes",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 363,
                                                                                    end: 370,
                                                                                    as_str(): "b.bytes",
                                                                                },
                                                                            },
                                                                            index: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 372,
                                                                                    end: 373,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 362,
                                                                        end: 374,
                                                                        as_str(): "(b.bytes)[0]",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: ArrayIndex(
                                                                        ArrayIndexExpression {
                                                                            prefix: Expression {
                                                                                kind: Subfield(
                                                                                    SubfieldExpression {
                                                                                        prefix: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                        ),
                                                                                                        start: 379,
                                                                                                        end: 386,
                                                                                                        as_str(): "other_b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 379,
                                                                                                end: 386,
                                                                                                as_str(): "other_b",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 387,
                                                                                                end: 392,
                                                                                                as_str(): "bytes",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 379,
                                                                                    end: 392,
                                                                                    as_str(): "other_b.bytes",
                                                                                },
                                                                            },
                                                                            index: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 395,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 378,
                                                                        end: 396,
                                                                        as_str(): "(other_b.bytes)[0]",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f57bed10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                        ),
                                                        start: 362,
                                                        end: 396,
                                                        as_str(): "(b.bytes)[0] != (other_b.bytes)[0]",
                                                    },
                                                },
                                                rhs: Expression {
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
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 415,
                                                                                    end: 417,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 415,
                                                                                    end: 417,
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
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 415,
                                                                                end: 417,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 415,
                                                                    end: 417,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: ArrayIndex(
                                                                        ArrayIndexExpression {
                                                                            prefix: Expression {
                                                                                kind: Subfield(
                                                                                    SubfieldExpression {
                                                                                        prefix: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                        ),
                                                                                                        start: 403,
                                                                                                        end: 404,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 403,
                                                                                                end: 404,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 405,
                                                                                                end: 410,
                                                                                                as_str(): "bytes",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 403,
                                                                                    end: 410,
                                                                                    as_str(): "b.bytes",
                                                                                },
                                                                            },
                                                                            index: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 412,
                                                                                    end: 413,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 402,
                                                                        end: 414,
                                                                        as_str(): "(b.bytes)[1]",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: ArrayIndex(
                                                                        ArrayIndexExpression {
                                                                            prefix: Expression {
                                                                                kind: Subfield(
                                                                                    SubfieldExpression {
                                                                                        prefix: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                        ),
                                                                                                        start: 419,
                                                                                                        end: 426,
                                                                                                        as_str(): "other_b",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 419,
                                                                                                end: 426,
                                                                                                as_str(): "other_b",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 427,
                                                                                                end: 432,
                                                                                                as_str(): "bytes",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 419,
                                                                                    end: 432,
                                                                                    as_str(): "other_b.bytes",
                                                                                },
                                                                            },
                                                                            index: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 434,
                                                                                    end: 435,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 418,
                                                                        end: 436,
                                                                        as_str(): "(other_b.bytes)[1]",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f57bed10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                        ),
                                                        start: 402,
                                                        end: 436,
                                                        as_str(): "(b.bytes)[1] == (other_b.bytes)[1]",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f57bed10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                            ),
                                            start: 361,
                                            end: 437,
                                            as_str(): "((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 361,
                                    end: 437,
                                    as_str(): "((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 95,
                            end: 439,
                            as_str(): "{\n    let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;\n    let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;\n\n    let b = B512::from((hi_bits, lo_bits));\n    let other_b = B512::new();\n    ((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb0f57bed10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                        ),
                        start: 77,
                        end: 439,
                        as_str(): "fn main() -> bool {\n    let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;\n    let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;\n\n    let b = B512::from((hi_bits, lo_bits));\n    let other_b = B512::new();\n    ((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb0f57bed10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                        ),
                        start: 90,
                        end: 94,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb0f57bed10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
            ),
            start: 77,
            end: 439,
            as_str(): "fn main() -> bool {\n    let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;\n    let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;\n\n    let b = B512::from((hi_bits, lo_bits));\n    let other_b = B512::new();\n    ((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])\n}",
        },
    },
]
