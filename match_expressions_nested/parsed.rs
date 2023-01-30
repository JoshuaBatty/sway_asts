[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe09612d790,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                            ),
                            start: 14,
                            end: 17,
                            as_str(): "Foo",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09612d790,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                    ),
                                    start: 24,
                                    end: 27,
                                    as_str(): "Bar",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 29,
                                        end: 33,
                                        as_str(): "Zoom",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09612d790,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                ),
                                start: 29,
                                end: 33,
                                as_str(): "Zoom",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe09612d790,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                ),
                                start: 24,
                                end: 33,
                                as_str(): "Bar: Zoom",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09612d790,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                        ),
                        start: 9,
                        end: 36,
                        as_str(): "enum Foo {\n    Bar: Zoom,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09612d790,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
            ),
            start: 9,
            end: 36,
            as_str(): "enum Foo {\n    Bar: Zoom,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe09612d790,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                            ),
                            start: 43,
                            end: 47,
                            as_str(): "Zoom",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09612d790,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                    ),
                                    start: 54,
                                    end: 57,
                                    as_str(): "Wow",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                ThirtyTwo,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe09612d790,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                ),
                                start: 59,
                                end: 62,
                                as_str(): "u32",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe09612d790,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                ),
                                start: 54,
                                end: 62,
                                as_str(): "Wow: u32",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09612d790,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                        ),
                        start: 38,
                        end: 65,
                        as_str(): "enum Zoom {\n    Wow: u32,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09612d790,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
            ),
            start: 38,
            end: 65,
            as_str(): "enum Zoom {\n    Wow: u32,\n}",
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
                            src (ptr): 0x00007fe09612d790,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                            ),
                            start: 70,
                            end: 74,
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
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 95,
                                                    as_str(): "x",
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
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 101,
                                                                                as_str(): "Foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 101,
                                                                            as_str(): "Foo",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 103,
                                                                            end: 106,
                                                                            as_str(): "Bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 106,
                                                                as_str(): "Foo::Bar",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
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
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 107,
                                                                                                end: 111,
                                                                                                as_str(): "Zoom",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 107,
                                                                                            end: 111,
                                                                                            as_str(): "Zoom",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 113,
                                                                                            end: 116,
                                                                                            as_str(): "Wow",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 107,
                                                                                end: 116,
                                                                                as_str(): "Zoom::Wow",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        123,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 117,
                                                                                    end: 120,
                                                                                    as_str(): "123",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 107,
                                                                    end: 121,
                                                                    as_str(): "Zoom::Wow(123)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 98,
                                                    end: 122,
                                                    as_str(): "Foo::Bar(Zoom::Wow(123))",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09612d790,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                    ),
                                    start: 90,
                                    end: 123,
                                    as_str(): "let x = Foo::Bar(Zoom::Wow(123));",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
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
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "x",
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
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 180,
                                                            as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
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
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 134,
                                                                                        end: 135,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 134,
                                                                                end: 135,
                                                                                as_str(): "x",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 146,
                                                                                                    end: 149,
                                                                                                    as_str(): "Foo",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 151,
                                                                                                end: 154,
                                                                                                as_str(): "Bar",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: EnumScrutinee {
                                                                                        call_path: CallPath {
                                                                                            prefixes: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 155,
                                                                                                        end: 159,
                                                                                                        as_str(): "Zoom",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 161,
                                                                                                    end: 164,
                                                                                                    as_str(): "Wow",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        value: Variable {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 165,
                                                                                                    end: 166,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 165,
                                                                                                end: 166,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 167,
                                                                                            as_str(): "Zoom::Wow(x)",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 146,
                                                                                        end: 168,
                                                                                        as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 173,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 174,
                                                                                    as_str(): "Foo::Bar(Zoom::Wow(x)) => x,",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 180,
                                                                    as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 180,
                                                            as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 128,
                                                    end: 180,
                                                    as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 180,
                                            as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09612d790,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                    ),
                                    start: 128,
                                    end: 180,
                                    as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09612d790,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                            ),
                            start: 84,
                            end: 182,
                            as_str(): "{\n    let x = Foo::Bar(Zoom::Wow(123));\n    match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe09612d790,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                        ),
                        start: 67,
                        end: 182,
                        as_str(): "fn main() -> u32 {\n    let x = Foo::Bar(Zoom::Wow(123));\n    match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09612d790,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                        ),
                        start: 80,
                        end: 83,
                        as_str(): "u32",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09612d790,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
            ),
            start: 67,
            end: 182,
            as_str(): "fn main() -> u32 {\n    let x = Foo::Bar(Zoom::Wow(123));\n    match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }\n}",
        },
    },
]
