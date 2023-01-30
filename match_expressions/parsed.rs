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
                            src (ptr): 0x00007fe0c8b9aa10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
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
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 36,
                                                    end: 37,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        5,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 40,
                                                    end: 41,
                                                    as_str(): "5",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 42,
                                    as_str(): "let x = 5;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 51,
                                                    end: 52,
                                                    as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "8",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        8,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "8",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 55,
                                                                    end: 152,
                                                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
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
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 61,
                                                                                                end: 62,
                                                                                                as_str(): "8",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 61,
                                                                                        end: 62,
                                                                                        as_str(): "8",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: Literal {
                                                                                            value: Numeric(
                                                                                                7,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 73,
                                                                                                end: 74,
                                                                                                as_str(): "7",
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
                                                                                                                            4,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 80,
                                                                                                                        end: 81,
                                                                                                                        as_str(): "4",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 80,
                                                                                                                end: 81,
                                                                                                                as_str(): "4",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 78,
                                                                                                        end: 83,
                                                                                                        as_str(): "{ 4 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 78,
                                                                                                end: 83,
                                                                                                as_str(): "{ 4 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 73,
                                                                                            end: 84,
                                                                                            as_str(): "7 => { 4 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: Literal {
                                                                                            value: Numeric(
                                                                                                9,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 93,
                                                                                                end: 94,
                                                                                                as_str(): "9",
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
                                                                                                                            5,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 100,
                                                                                                                        end: 101,
                                                                                                                        as_str(): "5",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 100,
                                                                                                                end: 101,
                                                                                                                as_str(): "5",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 98,
                                                                                                        end: 103,
                                                                                                        as_str(): "{ 5 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 98,
                                                                                                end: 103,
                                                                                                as_str(): "{ 5 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 93,
                                                                                            end: 104,
                                                                                            as_str(): "9 => { 5 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: Literal {
                                                                                            value: Numeric(
                                                                                                8,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 113,
                                                                                                end: 114,
                                                                                                as_str(): "8",
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
                                                                                                                            6,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 120,
                                                                                                                        end: 121,
                                                                                                                        as_str(): "6",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 120,
                                                                                                                end: 121,
                                                                                                                as_str(): "6",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 118,
                                                                                                        end: 123,
                                                                                                        as_str(): "{ 6 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 118,
                                                                                                end: 123,
                                                                                                as_str(): "{ 6 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 113,
                                                                                            end: 124,
                                                                                            as_str(): "8 => { 6 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 133,
                                                                                                end: 134,
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
                                                                                                                            100,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 140,
                                                                                                                        end: 143,
                                                                                                                        as_str(): "100",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 140,
                                                                                                                end: 143,
                                                                                                                as_str(): "100",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 138,
                                                                                                        end: 145,
                                                                                                        as_str(): "{ 100 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 138,
                                                                                                end: 145,
                                                                                                as_str(): "{ 100 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 133,
                                                                                            end: 146,
                                                                                            as_str(): "_ => { 100 },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 55,
                                                                            end: 152,
                                                                            as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 55,
                                                                    end: 152,
                                                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 55,
                                                            end: 152,
                                                            as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 152,
                                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 47,
                                    end: 153,
                                    as_str(): "let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 162,
                                                    end: 163,
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
                                                                                    "__match_return_var_name_2",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
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
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 173,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 166,
                                                                    end: 223,
                                                                    as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
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
                                                                                                "__match_return_var_name_2",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 173,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: Literal {
                                                                                            value: Numeric(
                                                                                                5,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 184,
                                                                                                end: 185,
                                                                                                as_str(): "5",
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
                                                                                                                            42,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 191,
                                                                                                                        end: 193,
                                                                                                                        as_str(): "42",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 191,
                                                                                                                end: 193,
                                                                                                                as_str(): "42",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 189,
                                                                                                        end: 195,
                                                                                                        as_str(): "{ 42 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 189,
                                                                                                end: 195,
                                                                                                as_str(): "{ 42 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 184,
                                                                                            end: 196,
                                                                                            as_str(): "5 => { 42 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 205,
                                                                                                end: 206,
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
                                                                                                                            24,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 212,
                                                                                                                        end: 214,
                                                                                                                        as_str(): "24",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 212,
                                                                                                                end: 214,
                                                                                                                as_str(): "24",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 210,
                                                                                                        end: 216,
                                                                                                        as_str(): "{ 24 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 210,
                                                                                                end: 216,
                                                                                                as_str(): "{ 24 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 205,
                                                                                            end: 217,
                                                                                            as_str(): "_ => { 24 },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 223,
                                                                            as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 166,
                                                                    end: 223,
                                                                    as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 223,
                                                            as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 223,
                                                    as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 224,
                                    as_str(): "let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };",
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
                                                                            "__match_return_var_name_3",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 235,
                                                                            end: 237,
                                                                            as_str(): "42",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 235,
                                                                            end: 237,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 229,
                                                            end: 290,
                                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
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
                                                                                        "__match_return_var_name_3",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 235,
                                                                                        end: 237,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 235,
                                                                                end: 237,
                                                                                as_str(): "42",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Numeric(
                                                                                        0,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 248,
                                                                                        end: 249,
                                                                                        as_str(): "0",
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
                                                                                                                    24,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 255,
                                                                                                                end: 257,
                                                                                                                as_str(): "24",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 255,
                                                                                                        end: 257,
                                                                                                        as_str(): "24",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 253,
                                                                                                end: 259,
                                                                                                as_str(): "{ 24 }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 253,
                                                                                        end: 259,
                                                                                        as_str(): "{ 24 }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 248,
                                                                                    end: 260,
                                                                                    as_str(): "0 => { 24 },",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Variable {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 269,
                                                                                            end: 272,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 269,
                                                                                        end: 272,
                                                                                        as_str(): "foo",
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
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 278,
                                                                                                                        end: 281,
                                                                                                                        as_str(): "foo",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 278,
                                                                                                                end: 281,
                                                                                                                as_str(): "foo",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 278,
                                                                                                        end: 281,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 276,
                                                                                                end: 283,
                                                                                                as_str(): "{ foo }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 276,
                                                                                        end: 283,
                                                                                        as_str(): "{ foo }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 269,
                                                                                    end: 284,
                                                                                    as_str(): "foo => { foo },",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 229,
                                                                    end: 290,
                                                                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 229,
                                                            end: 290,
                                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 290,
                                                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c8b9aa10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                            ),
                                            start: 229,
                                            end: 290,
                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 229,
                                    end: 290,
                                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c8b9aa10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                            ),
                            start: 26,
                            end: 292,
                            as_str(): "{\n    let x = 5;\n    let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0c8b9aa10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                        ),
                        start: 9,
                        end: 292,
                        as_str(): "fn main() -> u64 {\n    let x = 5;\n    let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c8b9aa10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c8b9aa10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
            ),
            start: 9,
            end: 292,
            as_str(): "fn main() -> u64 {\n    let x = 5;\n    let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }\n}",
        },
    },
]
