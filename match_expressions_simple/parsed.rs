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
                            src (ptr): 0x00007fe09ea4f9c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 36,
                                                    end: 37,
                                                    as_str(): "a",
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
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 42,
                                    as_str(): "let a = 5;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 51,
                                                    end: 52,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 61,
                                                                                                end: 62,
                                                                                                as_str(): "8",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 80,
                                                                                                                        end: 81,
                                                                                                                        as_str(): "4",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 80,
                                                                                                                end: 81,
                                                                                                                as_str(): "4",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 78,
                                                                                                        end: 83,
                                                                                                        as_str(): "{ 4 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 78,
                                                                                                end: 83,
                                                                                                as_str(): "{ 4 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 100,
                                                                                                                        end: 101,
                                                                                                                        as_str(): "5",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 100,
                                                                                                                end: 101,
                                                                                                                as_str(): "5",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 98,
                                                                                                        end: 103,
                                                                                                        as_str(): "{ 5 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 98,
                                                                                                end: 103,
                                                                                                as_str(): "{ 5 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 120,
                                                                                                                        end: 121,
                                                                                                                        as_str(): "6",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 120,
                                                                                                                end: 121,
                                                                                                                as_str(): "6",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 118,
                                                                                                        end: 123,
                                                                                                        as_str(): "{ 6 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 118,
                                                                                                end: 123,
                                                                                                as_str(): "{ 6 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 113,
                                                                                            end: 124,
                                                                                            as_str(): "8 => { 6 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 140,
                                                                                                                        end: 143,
                                                                                                                        as_str(): "100",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 140,
                                                                                                                end: 143,
                                                                                                                as_str(): "100",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 138,
                                                                                                        end: 145,
                                                                                                        as_str(): "{ 100 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 138,
                                                                                                end: 145,
                                                                                                as_str(): "{ 100 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 55,
                                                                            end: 152,
                                                                            as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 55,
                                                                    end: 152,
                                                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 55,
                                                            end: 152,
                                                            as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 47,
                                    end: 153,
                                    as_str(): "let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 162,
                                                    end: 163,
                                                    as_str(): "c",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 173,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 166,
                                                                    end: 223,
                                                                    as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 173,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: Literal {
                                                                                            value: Numeric(
                                                                                                5,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 191,
                                                                                                                        end: 193,
                                                                                                                        as_str(): "42",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 191,
                                                                                                                end: 193,
                                                                                                                as_str(): "42",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 189,
                                                                                                        end: 195,
                                                                                                        as_str(): "{ 42 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 189,
                                                                                                end: 195,
                                                                                                as_str(): "{ 42 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 184,
                                                                                            end: 196,
                                                                                            as_str(): "5 => { 42 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 212,
                                                                                                                        end: 214,
                                                                                                                        as_str(): "24",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 212,
                                                                                                                end: 214,
                                                                                                                as_str(): "24",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 210,
                                                                                                        end: 216,
                                                                                                        as_str(): "{ 24 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 210,
                                                                                                end: 216,
                                                                                                as_str(): "{ 24 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 223,
                                                                            as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 166,
                                                                    end: 223,
                                                                    as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 223,
                                                            as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 223,
                                                    as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 224,
                                    as_str(): "let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 233,
                                                    end: 234,
                                                    as_str(): "d",
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
                                                                                    "__match_return_var_name_3",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 243,
                                                                                    end: 245,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 243,
                                                                                    end: 245,
                                                                                    as_str(): "42",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 237,
                                                                    end: 298,
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 243,
                                                                                                end: 245,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 243,
                                                                                        end: 245,
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 256,
                                                                                                end: 257,
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 263,
                                                                                                                        end: 265,
                                                                                                                        as_str(): "24",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 263,
                                                                                                                end: 265,
                                                                                                                as_str(): "24",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 261,
                                                                                                        end: 267,
                                                                                                        as_str(): "{ 24 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 261,
                                                                                                end: 267,
                                                                                                as_str(): "{ 24 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 256,
                                                                                            end: 268,
                                                                                            as_str(): "0 => { 24 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: Variable {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 277,
                                                                                                    end: 280,
                                                                                                    as_str(): "foo",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 277,
                                                                                                end: 280,
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
                                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 286,
                                                                                                                                end: 289,
                                                                                                                                as_str(): "foo",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 286,
                                                                                                                        end: 289,
                                                                                                                        as_str(): "foo",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 286,
                                                                                                                end: 289,
                                                                                                                as_str(): "foo",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 284,
                                                                                                        end: 291,
                                                                                                        as_str(): "{ foo }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 284,
                                                                                                end: 291,
                                                                                                as_str(): "{ foo }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 277,
                                                                                            end: 292,
                                                                                            as_str(): "foo => { foo },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 237,
                                                                            end: 298,
                                                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 237,
                                                                    end: 298,
                                                                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 237,
                                                            end: 298,
                                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 237,
                                                    end: 298,
                                                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 229,
                                    end: 299,
                                    as_str(): "let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 308,
                                                    end: 309,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Tuple(
                                                                [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 323,
                                                                            end: 324,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 326,
                                                                            end: 327,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 322,
                                                                end: 328,
                                                                as_str(): "(1, 2)",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: Tuple(
                                                                [
                                                                    Expression {
                                                                        kind: Tuple(
                                                                            [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 353,
                                                                                        end: 354,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            4,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 356,
                                                                                        end: 357,
                                                                                        as_str(): "4",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 352,
                                                                            end: 358,
                                                                            as_str(): "(3, 4)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 372,
                                                                            end: 373,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 338,
                                                                end: 383,
                                                                as_str(): "(\n            (3, 4),\n            5\n        )",
                                                            },
                                                        },
                                                    ],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 312,
                                                    end: 389,
                                                    as_str(): "(\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 304,
                                    end: 390,
                                    as_str(): "let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 399,
                                                    end: 400,
                                                    as_str(): "f",
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
                                                                                    "__match_return_var_name_4",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 409,
                                                                                    end: 410,
                                                                                    as_str(): "e",
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 409,
                                                                                            end: 410,
                                                                                            as_str(): "e",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 409,
                                                                                    end: 410,
                                                                                    as_str(): "e",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 403,
                                                                    end: 500,
                                                                    as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
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
                                                                                                "__match_return_var_name_4",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 409,
                                                                                                end: 410,
                                                                                                as_str(): "e",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 409,
                                                                                        end: 410,
                                                                                        as_str(): "e",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: Tuple {
                                                                                            elems: [
                                                                                                Tuple {
                                                                                                    elems: [
                                                                                                        Literal {
                                                                                                            value: Numeric(
                                                                                                                3,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 423,
                                                                                                                end: 424,
                                                                                                                as_str(): "3",
                                                                                                            },
                                                                                                        },
                                                                                                        CatchAll {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 426,
                                                                                                                end: 427,
                                                                                                                as_str(): "_",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 422,
                                                                                                        end: 428,
                                                                                                        as_str(): "(3, _)",
                                                                                                    },
                                                                                                },
                                                                                                CatchAll {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 430,
                                                                                                        end: 431,
                                                                                                        as_str(): "_",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 421,
                                                                                                end: 432,
                                                                                                as_str(): "((3, _), _)",
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
                                                                                                                            99,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 438,
                                                                                                                        end: 440,
                                                                                                                        as_str(): "99",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 438,
                                                                                                                end: 440,
                                                                                                                as_str(): "99",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 436,
                                                                                                        end: 442,
                                                                                                        as_str(): "{ 99 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 436,
                                                                                                end: 442,
                                                                                                as_str(): "{ 99 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 421,
                                                                                            end: 443,
                                                                                            as_str(): "((3, _), _) => { 99 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: Tuple {
                                                                                            elems: [
                                                                                                CatchAll {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 453,
                                                                                                        end: 454,
                                                                                                        as_str(): "_",
                                                                                                    },
                                                                                                },
                                                                                                Tuple {
                                                                                                    elems: [
                                                                                                        CatchAll {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 457,
                                                                                                                end: 458,
                                                                                                                as_str(): "_",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal {
                                                                                                            value: Numeric(
                                                                                                                5,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 460,
                                                                                                                end: 461,
                                                                                                                as_str(): "5",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 456,
                                                                                                        end: 462,
                                                                                                        as_str(): "(_, 5)",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 452,
                                                                                                end: 463,
                                                                                                as_str(): "(_, (_, 5))",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 469,
                                                                                                                        end: 471,
                                                                                                                        as_str(): "42",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 469,
                                                                                                                end: 471,
                                                                                                                as_str(): "42",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 467,
                                                                                                        end: 473,
                                                                                                        as_str(): "{ 42 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 467,
                                                                                                end: 473,
                                                                                                as_str(): "{ 42 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 452,
                                                                                            end: 474,
                                                                                            as_str(): "(_, (_, 5)) => { 42 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 483,
                                                                                                end: 484,
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 490,
                                                                                                                        end: 491,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 490,
                                                                                                                end: 491,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 488,
                                                                                                        end: 493,
                                                                                                        as_str(): "{ 0 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 488,
                                                                                                end: 493,
                                                                                                as_str(): "{ 0 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 483,
                                                                                            end: 494,
                                                                                            as_str(): "_ => { 0 },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 403,
                                                                            end: 500,
                                                                            as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 403,
                                                                    end: 500,
                                                                    as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 403,
                                                            end: 500,
                                                            as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 403,
                                                    end: 500,
                                                    as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 395,
                                    end: 501,
                                    as_str(): "let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };",
                                },
                            },
                            AstNode {
                                content: Expression(
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
                                                                            "__match_return_var_name_5",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 513,
                                                                            end: 517,
                                                                            as_str(): "true",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 513,
                                                                            end: 517,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 623,
                                                            as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
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
                                                                                        "__match_return_var_name_5",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 513,
                                                                                        end: 517,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 513,
                                                                                end: 517,
                                                                                as_str(): "true",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Boolean(
                                                                                        true,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 528,
                                                                                        end: 532,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Tuple(
                                                                                        [],
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 536,
                                                                                        end: 538,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 528,
                                                                                    end: 539,
                                                                                    as_str(): "true => (),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Boolean(
                                                                                        false,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 548,
                                                                                        end: 553,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Tuple(
                                                                                        [],
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 557,
                                                                                        end: 559,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 548,
                                                                                    end: 560,
                                                                                    as_str(): "false => (),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Variable {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 569,
                                                                                            end: 572,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 569,
                                                                                        end: 572,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Tuple(
                                                                                        [],
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 576,
                                                                                        end: 578,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 569,
                                                                                    end: 579,
                                                                                    as_str(): "foo => (),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 507,
                                                                    end: 623,
                                                                    as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 623,
                                                            as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 507,
                                                    end: 623,
                                                    as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 507,
                                            end: 623,
                                            as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 507,
                                    end: 623,
                                    as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: LazyOperator(
                                                        LazyOperatorExpression {
                                                            op: And,
                                                            lhs: Expression {
                                                                kind: LazyOperator(
                                                                    LazyOperatorExpression {
                                                                        op: And,
                                                                        lhs: Expression {
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 634,
                                                                                                                        end: 636,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 634,
                                                                                                                        end: 636,
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
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 634,
                                                                                                                    end: 636,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 634,
                                                                                                        end: 636,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 632,
                                                                                                                    end: 633,
                                                                                                                    as_str(): "b",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 632,
                                                                                                            end: 633,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                6,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 637,
                                                                                                            end: 638,
                                                                                                            as_str(): "6",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 632,
                                                                                            end: 638,
                                                                                            as_str(): "b == 6",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 644,
                                                                                                                        end: 646,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 644,
                                                                                                                        end: 646,
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
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 644,
                                                                                                                    end: 646,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 644,
                                                                                                        end: 646,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 642,
                                                                                                                    end: 643,
                                                                                                                    as_str(): "c",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 642,
                                                                                                            end: 643,
                                                                                                            as_str(): "c",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                42,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 647,
                                                                                                            end: 649,
                                                                                                            as_str(): "42",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 642,
                                                                                            end: 649,
                                                                                            as_str(): "c == 42",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 632,
                                                                                end: 649,
                                                                                as_str(): "b == 6 && c == 42",
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 655,
                                                                                                            end: 657,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 655,
                                                                                                            end: 657,
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 655,
                                                                                                        end: 657,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: true,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 655,
                                                                                            end: 657,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 653,
                                                                                                        end: 654,
                                                                                                        as_str(): "d",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 653,
                                                                                                end: 654,
                                                                                                as_str(): "d",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    42,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 658,
                                                                                                end: 660,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 653,
                                                                                end: 660,
                                                                                as_str(): "d == 42",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 632,
                                                                    end: 660,
                                                                    as_str(): "b == 6 && c == 42 && d == 42",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 666,
                                                                                                end: 668,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 666,
                                                                                                end: 668,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 666,
                                                                                            end: 668,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 666,
                                                                                end: 668,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 664,
                                                                                            end: 665,
                                                                                            as_str(): "f",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 664,
                                                                                    end: 665,
                                                                                    as_str(): "f",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        42,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 669,
                                                                                    end: 671,
                                                                                    as_str(): "42",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 664,
                                                                    end: 671,
                                                                    as_str(): "f == 42",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                        ),
                                                        start: 632,
                                                        end: 671,
                                                        as_str(): "b == 6 && c == 42 && d == 42 && f == 42",
                                                    },
                                                },
                                                then: Expression {
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 682,
                                                                                end: 684,
                                                                                as_str(): "42",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 682,
                                                                        end: 684,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 672,
                                                                end: 690,
                                                                as_str(): "{\n        42\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                        ),
                                                        start: 672,
                                                        end: 690,
                                                        as_str(): "{\n        42\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 706,
                                                                                    end: 707,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 706,
                                                                            end: 707,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 696,
                                                                    end: 713,
                                                                    as_str(): "{\n        0\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 696,
                                                            end: 713,
                                                            as_str(): "{\n        0\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 629,
                                            end: 713,
                                            as_str(): "if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 629,
                                    end: 713,
                                    as_str(): "if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09ea4f9c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                            ),
                            start: 26,
                            end: 715,
                            as_str(): "{\n    let a = 5;\n    let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };\n    let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );\n    let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };\n\n    match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }\n\n    if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe09ea4f9c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                        ),
                        start: 9,
                        end: 715,
                        as_str(): "fn main() -> u64 {\n    let a = 5;\n    let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };\n    let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );\n    let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };\n\n    match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }\n\n    if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09ea4f9c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09ea4f9c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
            ),
            start: 9,
            end: 715,
            as_str(): "fn main() -> u64 {\n    let a = 5;\n    let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };\n    let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );\n    let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };\n\n    match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }\n\n    if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }\n}",
        },
    },
]
