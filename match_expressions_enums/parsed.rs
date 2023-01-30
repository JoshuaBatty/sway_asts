[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0a9b0a660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                            ),
                            start: 14,
                            end: 15,
                            as_str(): "X",
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
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 22,
                                    end: 23,
                                    as_str(): "Y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0a9b0a660,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                ),
                                start: 25,
                                end: 28,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0a9b0a660,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                ),
                                start: 22,
                                end: 28,
                                as_str(): "Y: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 34,
                                    end: 35,
                                    as_str(): "Z",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007fe0a9b0a660,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                ),
                                start: 37,
                                end: 41,
                                as_str(): "bool",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0a9b0a660,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                ),
                                start: 34,
                                end: 41,
                                as_str(): "Z: bool",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0a9b0a660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                        ),
                        start: 9,
                        end: 43,
                        as_str(): "enum X {\n    Y: u64,\n    Z: bool\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a9b0a660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
            ),
            start: 9,
            end: 43,
            as_str(): "enum X {\n    Y: u64,\n    Z: bool\n}",
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
                            src (ptr): 0x00007fe0a9b0a660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                            ),
                            start: 48,
                            end: 52,
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
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 72,
                                                    end: 73,
                                                    as_str(): "a",
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
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 76,
                                                                                end: 77,
                                                                                as_str(): "X",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 76,
                                                                            end: 77,
                                                                            as_str(): "X",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 79,
                                                                            end: 80,
                                                                            as_str(): "Y",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 80,
                                                                as_str(): "X::Y",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        42,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 83,
                                                                    as_str(): "42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 76,
                                                    end: 84,
                                                    as_str(): "X::Y(42)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 85,
                                    as_str(): "let a = X::Y(42);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 95,
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
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 105,
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
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 104,
                                                                                            end: 105,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 105,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 98,
                                                                    end: 191,
                                                                    as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
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
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 104,
                                                                                                end: 105,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 104,
                                                                                        end: 105,
                                                                                        as_str(): "a",
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
                                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 116,
                                                                                                            end: 117,
                                                                                                            as_str(): "X",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 119,
                                                                                                        end: 120,
                                                                                                        as_str(): "Y",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: Variable {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 121,
                                                                                                        end: 123,
                                                                                                        as_str(): "hi",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 121,
                                                                                                    end: 123,
                                                                                                    as_str(): "hi",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 116,
                                                                                                end: 124,
                                                                                                as_str(): "X::Y(hi)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 130,
                                                                                                                                end: 132,
                                                                                                                                as_str(): "hi",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 130,
                                                                                                                        end: 132,
                                                                                                                        as_str(): "hi",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 130,
                                                                                                                end: 132,
                                                                                                                as_str(): "hi",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 128,
                                                                                                        end: 134,
                                                                                                        as_str(): "{ hi }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 128,
                                                                                                end: 134,
                                                                                                as_str(): "{ hi }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 116,
                                                                                            end: 135,
                                                                                            as_str(): "X::Y(hi) => { hi },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: EnumScrutinee {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 144,
                                                                                                            end: 145,
                                                                                                            as_str(): "X",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 147,
                                                                                                        end: 148,
                                                                                                        as_str(): "Z",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: Literal {
                                                                                                value: Boolean(
                                                                                                    false,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 149,
                                                                                                    end: 154,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 144,
                                                                                                end: 155,
                                                                                                as_str(): "X::Z(false)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            0,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 161,
                                                                                                                        end: 162,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 161,
                                                                                                                end: 162,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 159,
                                                                                                        end: 164,
                                                                                                        as_str(): "{ 0 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 159,
                                                                                                end: 164,
                                                                                                as_str(): "{ 0 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 144,
                                                                                            end: 165,
                                                                                            as_str(): "X::Z(false) => { 0 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 174,
                                                                                                end: 175,
                                                                                                as_str(): "_",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            0,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 181,
                                                                                                                        end: 182,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 181,
                                                                                                                end: 182,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 179,
                                                                                                        end: 184,
                                                                                                        as_str(): "{ 0 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 179,
                                                                                                end: 184,
                                                                                                as_str(): "{ 0 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 174,
                                                                                            end: 185,
                                                                                            as_str(): "_ => { 0 },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 191,
                                                                            as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 98,
                                                                    end: 191,
                                                                    as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 191,
                                                            as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 98,
                                                    end: 191,
                                                    as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 90,
                                    end: 192,
                                    as_str(): "let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 202,
                                                    end: 203,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 202,
                                            end: 203,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 202,
                                    end: 203,
                                    as_str(): "b",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0a9b0a660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                            ),
                            start: 62,
                            end: 205,
                            as_str(): "{\n    let a = X::Y(42);\n    let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };\n    \n    b\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0a9b0a660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                        ),
                        start: 45,
                        end: 205,
                        as_str(): "fn main() -> u64 {\n    let a = X::Y(42);\n    let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };\n    \n    b\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0a9b0a660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                        ),
                        start: 58,
                        end: 61,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a9b0a660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
            ),
            start: 45,
            end: 205,
            as_str(): "fn main() -> u64 {\n    let a = X::Y(42);\n    let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };\n    \n    b\n}",
        },
    },
]
