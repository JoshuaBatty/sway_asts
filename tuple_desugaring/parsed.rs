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
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                            ),
                            start: 12,
                            end: 24,
                            as_str(): "gimme_a_pair",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Tuple(
                                            [
                                                Expression {
                                                    kind: Literal(
                                                        U32(
                                                            1,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe03346fb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                        ),
                                                        start: 48,
                                                        end: 52,
                                                        as_str(): "1u32",
                                                    },
                                                },
                                                Expression {
                                                    kind: Literal(
                                                        U64(
                                                            2,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe03346fb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                        ),
                                                        start: 54,
                                                        end: 58,
                                                        as_str(): "2u64",
                                                    },
                                                },
                                            ],
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 59,
                                            as_str(): "(1u32, 2u64)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 47,
                                    end: 59,
                                    as_str(): "(1u32, 2u64)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                            ),
                            start: 41,
                            end: 61,
                            as_str(): "{\n    (1u32, 2u64)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe03346fb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                        ),
                        start: 9,
                        end: 61,
                        as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
                    },
                    return_type: Tuple(
                        [
                            TypeArgument {
                                type_id: TypeId(
                                    33,
                                ),
                                initial_type_id: TypeId(
                                    33,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 31,
                                    end: 34,
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
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 39,
                                    as_str(): "u64",
                                },
                            },
                        ],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe03346fb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                        ),
                        start: 30,
                        end: 40,
                        as_str(): "(u32, u64)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03346fb00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
            ),
            start: 9,
            end: 61,
            as_str(): "fn gimme_a_pair() -> (u32, u64) {\n    (1u32, 2u64)\n}",
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
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 90,
                                                    end: 91,
                                                    as_str(): "x",
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
                                                                        src (ptr): 0x00007fe03346fb00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                        ),
                                                                        start: 94,
                                                                        end: 106,
                                                                        as_str(): "gimme_a_pair",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 106,
                                                                as_str(): "gimme_a_pair",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 108,
                                                    as_str(): "gimme_a_pair()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 109,
                                    as_str(): "let x = gimme_a_pair();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 119,
                                                    as_str(): "y",
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
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 128,
                                                                                    end: 129,
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
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 128,
                                                                                            end: 129,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 128,
                                                                                    end: 129,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 209,
                                                                    as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
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
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 128,
                                                                                                end: 129,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 128,
                                                                                        end: 129,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: Tuple {
                                                                                            elems: [
                                                                                                Variable {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 141,
                                                                                                            end: 142,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 141,
                                                                                                        end: 142,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                                Literal {
                                                                                                    value: U64(
                                                                                                        3,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 144,
                                                                                                        end: 148,
                                                                                                        as_str(): "3u64",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 140,
                                                                                                end: 149,
                                                                                                as_str(): "(a, 3u64)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Tuple(
                                                                                                                        [
                                                                                                                            Expression {
                                                                                                                                kind: Variable(
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 156,
                                                                                                                                            end: 157,
                                                                                                                                            as_str(): "a",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 156,
                                                                                                                                    end: 157,
                                                                                                                                    as_str(): "a",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            Expression {
                                                                                                                                kind: Literal(
                                                                                                                                    U32(
                                                                                                                                        7,
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 159,
                                                                                                                                    end: 163,
                                                                                                                                    as_str(): "7u32",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ],
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 155,
                                                                                                                        end: 164,
                                                                                                                        as_str(): "(a, 7u32)",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 155,
                                                                                                                end: 164,
                                                                                                                as_str(): "(a, 7u32)",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 153,
                                                                                                        end: 166,
                                                                                                        as_str(): "{ (a, 7u32) }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 153,
                                                                                                end: 166,
                                                                                                as_str(): "{ (a, 7u32) }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 140,
                                                                                            end: 167,
                                                                                            as_str(): "(a, 3u64) => { (a, 7u32) },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: Tuple {
                                                                                            elems: [
                                                                                                Variable {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 177,
                                                                                                            end: 178,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 177,
                                                                                                        end: 178,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                                Variable {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 180,
                                                                                                            end: 181,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 180,
                                                                                                        end: 181,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 176,
                                                                                                end: 182,
                                                                                                as_str(): "(a, b)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Tuple(
                                                                                                                        [
                                                                                                                            Expression {
                                                                                                                                kind: Literal(
                                                                                                                                    U32(
                                                                                                                                        0,
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 189,
                                                                                                                                    end: 193,
                                                                                                                                    as_str(): "0u32",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            Expression {
                                                                                                                                kind: Literal(
                                                                                                                                    U32(
                                                                                                                                        9,
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 195,
                                                                                                                                    end: 199,
                                                                                                                                    as_str(): "9u32",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ],
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 188,
                                                                                                                        end: 200,
                                                                                                                        as_str(): "(0u32, 9u32)",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 188,
                                                                                                                end: 200,
                                                                                                                as_str(): "(0u32, 9u32)",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 186,
                                                                                                        end: 202,
                                                                                                        as_str(): "{ (0u32, 9u32) }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 186,
                                                                                                end: 202,
                                                                                                as_str(): "{ (0u32, 9u32) }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 176,
                                                                                            end: 203,
                                                                                            as_str(): "(a, b) => { (0u32, 9u32) },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 122,
                                                                            end: 209,
                                                                            as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 209,
                                                                    as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 122,
                                                            end: 209,
                                                            as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 209,
                                                    as_str(): "match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 210,
                                    as_str(): "let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };",
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
                                                                            "__match_return_var_name_2",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 221,
                                                                            end: 222,
                                                                            as_str(): "y",
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
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 221,
                                                                                    end: 222,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 221,
                                                                            end: 222,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 215,
                                                            end: 255,
                                                            as_str(): "match y {\n        (a, b) => { b },\n    }",
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
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 221,
                                                                                        end: 222,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 221,
                                                                                end: 222,
                                                                                as_str(): "y",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Tuple {
                                                                                    elems: [
                                                                                        Variable {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 234,
                                                                                                    end: 235,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 234,
                                                                                                end: 235,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                        },
                                                                                        Variable {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 237,
                                                                                                    end: 238,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 237,
                                                                                                end: 238,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 239,
                                                                                        as_str(): "(a, b)",
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
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 245,
                                                                                                                        end: 246,
                                                                                                                        as_str(): "b",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 245,
                                                                                                                end: 246,
                                                                                                                as_str(): "b",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 245,
                                                                                                        end: 246,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 243,
                                                                                                end: 248,
                                                                                                as_str(): "{ b }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 243,
                                                                                        end: 248,
                                                                                        as_str(): "{ b }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 233,
                                                                                    end: 249,
                                                                                    as_str(): "(a, b) => { b },",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 215,
                                                                    end: 255,
                                                                    as_str(): "match y {\n        (a, b) => { b },\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 215,
                                                            end: 255,
                                                            as_str(): "match y {\n        (a, b) => { b },\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 215,
                                                    end: 255,
                                                    as_str(): "match y {\n        (a, b) => { b },\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 215,
                                            end: 255,
                                            as_str(): "match y {\n        (a, b) => { b },\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03346fb00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                    ),
                                    start: 215,
                                    end: 255,
                                    as_str(): "match y {\n        (a, b) => { b },\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                            ),
                            start: 80,
                            end: 257,
                            as_str(): "{\n    let x = gimme_a_pair();\n    let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };\n    match y {\n        (a, b) => { b },\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe03346fb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                        ),
                        start: 63,
                        end: 257,
                        as_str(): "fn main() -> u32 {\n    let x = gimme_a_pair();\n    let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };\n    match y {\n        (a, b) => { b },\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        ThirtyTwo,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe03346fb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                        ),
                        start: 76,
                        end: 79,
                        as_str(): "u32",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe03346fb00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
            ),
            start: 63,
            end: 257,
            as_str(): "fn main() -> u32 {\n    let x = gimme_a_pair();\n    let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };\n    match y {\n        (a, b) => { b },\n    }\n}",
        },
    },
]
