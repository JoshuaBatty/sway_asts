[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 9,
            end: 28,
            as_str(): "use std::assert::*;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 33,
                            end: 36,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 38,
                            end: 44,
                            as_str(): "revert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 29,
            end: 48,
            as_str(): "use std::revert::*;",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 53,
                            end: 64,
                            as_str(): "third_match",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 94,
                                                                            end: 99,
                                                                            as_str(): "value",
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 94,
                                                                                    end: 99,
                                                                                    as_str(): "value",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 94,
                                                                            end: 99,
                                                                            as_str(): "value",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 88,
                                                            end: 121,
                                                            as_str(): "match value {\n    foo => 5u8,\n  }",
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 94,
                                                                                        end: 99,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 94,
                                                                                end: 99,
                                                                                as_str(): "value",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Variable {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 106,
                                                                                            end: 109,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 106,
                                                                                        end: 109,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        U8(
                                                                                            5,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 113,
                                                                                        end: 116,
                                                                                        as_str(): "5u8",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 106,
                                                                                    end: 117,
                                                                                    as_str(): "foo => 5u8,",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 121,
                                                                    as_str(): "match value {\n    foo => 5u8,\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 88,
                                                            end: 121,
                                                            as_str(): "match value {\n    foo => 5u8,\n  }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 88,
                                                    end: 121,
                                                    as_str(): "match value {\n    foo => 5u8,\n  }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 121,
                                            as_str(): "match value {\n    foo => 5u8,\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 88,
                                    end: 121,
                                    as_str(): "match value {\n    foo => 5u8,\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 84,
                            end: 123,
                            as_str(): "{\n  match value {\n    foo => 5u8,\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 73,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 76,
                                        as_str(): "A",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 75,
                                end: 76,
                                as_str(): "A",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 50,
                        end: 123,
                        as_str(): "fn third_match<A>(value: A) -> u8 {\n  match value {\n    foo => 5u8,\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        Eight,
                    ),
                    type_parameters: [
                        A: TypeId(31628),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 81,
                        end: 83,
                        as_str(): "u8",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 50,
            end: 123,
            as_str(): "fn third_match<A>(value: A) -> u8 {\n  match value {\n    foo => 5u8,\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 128,
                            end: 140,
                            as_str(): "second_match",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 190,
                                                                            as_str(): "third_match(value)",
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
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 172,
                                                                                                end: 183,
                                                                                                as_str(): "third_match",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 183,
                                                                                        as_str(): "third_match",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 184,
                                                                                                    end: 189,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 184,
                                                                                            end: 189,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 190,
                                                                            as_str(): "third_match(value)",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 301,
                                                            as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 190,
                                                                                        as_str(): "third_match(value)",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 172,
                                                                                end: 190,
                                                                                as_str(): "third_match(value)",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: U8(
                                                                                        1,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 197,
                                                                                        end: 200,
                                                                                        as_str(): "1u8",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            false,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 204,
                                                                                        end: 209,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 197,
                                                                                    end: 210,
                                                                                    as_str(): "1u8 => false,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: U8(
                                                                                        2,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 215,
                                                                                        end: 218,
                                                                                        as_str(): "2u8",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            false,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 222,
                                                                                        end: 227,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 215,
                                                                                    end: 228,
                                                                                    as_str(): "2u8 => false,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: U8(
                                                                                        3,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 236,
                                                                                        as_str(): "3u8",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            false,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 240,
                                                                                        end: 245,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 233,
                                                                                    end: 246,
                                                                                    as_str(): "3u8 => false,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: U8(
                                                                                        4,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 251,
                                                                                        end: 254,
                                                                                        as_str(): "4u8",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            false,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 258,
                                                                                        end: 263,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 251,
                                                                                    end: 264,
                                                                                    as_str(): "4u8 => false,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: U8(
                                                                                        5,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 269,
                                                                                        end: 272,
                                                                                        as_str(): "5u8",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 276,
                                                                                        end: 280,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 269,
                                                                                    end: 281,
                                                                                    as_str(): "5u8 => true,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: CatchAll {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 286,
                                                                                        end: 287,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            false,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 291,
                                                                                        end: 296,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 286,
                                                                                    end: 297,
                                                                                    as_str(): "_ => false,",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 166,
                                                                    end: 301,
                                                                    as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 301,
                                                            as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 301,
                                                    as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 166,
                                            end: 301,
                                            as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 301,
                                    as_str(): "match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 162,
                            end: 303,
                            as_str(): "{\n  match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 144,
                                    end: 149,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 152,
                                        as_str(): "B",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 151,
                                end: 152,
                                as_str(): "B",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 125,
                        end: 303,
                        as_str(): "fn second_match<B>(value: B) -> bool {\n  match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [
                        B: TypeId(31629),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 157,
                        end: 161,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 125,
            end: 303,
            as_str(): "fn second_match<B>(value: B) -> bool {\n  match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 308,
                            end: 319,
                            as_str(): "first_match",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 350,
                                                                            end: 369,
                                                                            as_str(): "second_match(value)",
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
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 350,
                                                                                                end: 362,
                                                                                                as_str(): "second_match",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 350,
                                                                                        end: 362,
                                                                                        as_str(): "second_match",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 363,
                                                                                                    end: 368,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 363,
                                                                                            end: 368,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 350,
                                                                            end: 369,
                                                                            as_str(): "second_match(value)",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 412,
                                                            as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 350,
                                                                                        end: 369,
                                                                                        as_str(): "second_match(value)",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 350,
                                                                                end: 369,
                                                                                as_str(): "second_match(value)",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Boolean(
                                                                                        false,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 376,
                                                                                        end: 381,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 385,
                                                                                        end: 389,
                                                                                        as_str(): "2u64",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 390,
                                                                                    as_str(): "false => 2u64,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Boolean(
                                                                                        true,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 395,
                                                                                        end: 399,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 403,
                                                                                        end: 407,
                                                                                        as_str(): "3u64",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 408,
                                                                                    as_str(): "true => 3u64,",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 344,
                                                                    end: 412,
                                                                    as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 412,
                                                            as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 344,
                                                    end: 412,
                                                    as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 344,
                                            end: 412,
                                            as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 344,
                                    end: 412,
                                    as_str(): "match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 340,
                            end: 414,
                            as_str(): "{\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 323,
                                    end: 328,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 330,
                                        end: 331,
                                        as_str(): "C",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 330,
                                end: 331,
                                as_str(): "C",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 305,
                        end: 414,
                        as_str(): "fn first_match<C>(value: C) -> u64 {\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [
                        C: TypeId(31630),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 336,
                        end: 339,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 305,
            end: 414,
            as_str(): "fn first_match<C>(value: C) -> u64 {\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 419,
                            end: 427,
                            as_str(): "third_if",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 454,
                                                        end: 458,
                                                        as_str(): "true",
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
                                                                                U8(
                                                                                    5,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 465,
                                                                                end: 468,
                                                                                as_str(): "5u8",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 465,
                                                                        end: 468,
                                                                        as_str(): "5u8",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 459,
                                                                end: 472,
                                                                as_str(): "{\n    5u8\n  }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 459,
                                                        end: 472,
                                                        as_str(): "{\n    5u8\n  }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
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
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 484,
                                                                                                        end: 490,
                                                                                                        as_str(): "revert",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 484,
                                                                                                end: 490,
                                                                                                as_str(): "revert",
                                                                                            },
                                                                                        },
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 491,
                                                                                                    end: 492,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 484,
                                                                                    end: 493,
                                                                                    as_str(): "revert(0)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 493,
                                                                            as_str(): "revert(0)",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 478,
                                                                    end: 498,
                                                                    as_str(): "{\n    revert(0);\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 478,
                                                            end: 498,
                                                            as_str(): "{\n    revert(0);\n  }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 451,
                                            end: 498,
                                            as_str(): "if true {\n    5u8\n  } else {\n    revert(0);\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 451,
                                    end: 498,
                                    as_str(): "if true {\n    5u8\n  } else {\n    revert(0);\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 447,
                            end: 500,
                            as_str(): "{\n  if true {\n    5u8\n  } else {\n    revert(0);\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 431,
                                    end: 436,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 438,
                                        end: 439,
                                        as_str(): "D",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 438,
                                end: 439,
                                as_str(): "D",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 416,
                        end: 500,
                        as_str(): "fn third_if<D>(value: D) -> u8 {\n  if true {\n    5u8\n  } else {\n    revert(0);\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        Eight,
                    ),
                    type_parameters: [
                        D: TypeId(31631),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 444,
                        end: 446,
                        as_str(): "u8",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 416,
            end: 500,
            as_str(): "fn third_if<D>(value: D) -> u8 {\n  if true {\n    5u8\n  } else {\n    revert(0);\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 505,
                            end: 514,
                            as_str(): "second_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 544,
                                                    end: 549,
                                                    as_str(): "third",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 552,
                                                                        end: 560,
                                                                        as_str(): "third_if",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 552,
                                                                end: 560,
                                                                as_str(): "third_if",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 561,
                                                                            end: 566,
                                                                            as_str(): "value",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 561,
                                                                    end: 566,
                                                                    as_str(): "value",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 552,
                                                    end: 567,
                                                    as_str(): "third_if(value)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 540,
                                    end: 568,
                                    as_str(): "let third = third_if(value);",
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
                                                            op: Or,
                                                            lhs: Expression {
                                                                kind: LazyOperator(
                                                                    LazyOperatorExpression {
                                                                        op: Or,
                                                                        lhs: Expression {
                                                                            kind: LazyOperator(
                                                                                LazyOperatorExpression {
                                                                                    op: Or,
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
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 580,
                                                                                                                        end: 582,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 580,
                                                                                                                        end: 582,
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
                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 580,
                                                                                                                    end: 582,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 580,
                                                                                                        end: 582,
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
                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 574,
                                                                                                                    end: 579,
                                                                                                                    as_str(): "third",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 574,
                                                                                                            end: 579,
                                                                                                            as_str(): "third",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            U8(
                                                                                                                1,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 583,
                                                                                                            end: 586,
                                                                                                            as_str(): "1u8",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 574,
                                                                                            end: 586,
                                                                                            as_str(): "third == 1u8",
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
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 596,
                                                                                                                        end: 598,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 596,
                                                                                                                        end: 598,
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
                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 596,
                                                                                                                    end: 598,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 596,
                                                                                                        end: 598,
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
                                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 590,
                                                                                                                    end: 595,
                                                                                                                    as_str(): "third",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 590,
                                                                                                            end: 595,
                                                                                                            as_str(): "third",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            U8(
                                                                                                                2,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 599,
                                                                                                            end: 602,
                                                                                                            as_str(): "2u8",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 590,
                                                                                            end: 602,
                                                                                            as_str(): "third == 2u8",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 574,
                                                                                end: 602,
                                                                                as_str(): "third == 1u8 || third == 2u8",
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
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 612,
                                                                                                            end: 614,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 612,
                                                                                                            end: 614,
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
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 612,
                                                                                                        end: 614,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: true,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 612,
                                                                                            end: 614,
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
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 606,
                                                                                                        end: 611,
                                                                                                        as_str(): "third",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 606,
                                                                                                end: 611,
                                                                                                as_str(): "third",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                U8(
                                                                                                    3,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 615,
                                                                                                end: 618,
                                                                                                as_str(): "3u8",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 606,
                                                                                end: 618,
                                                                                as_str(): "third == 3u8",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 574,
                                                                    end: 618,
                                                                    as_str(): "third == 1u8 || third == 2u8 || third == 3u8",
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
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 628,
                                                                                                end: 630,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 628,
                                                                                                end: 630,
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
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 628,
                                                                                            end: 630,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 628,
                                                                                end: 630,
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
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 622,
                                                                                            end: 627,
                                                                                            as_str(): "third",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 622,
                                                                                    end: 627,
                                                                                    as_str(): "third",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        4,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 631,
                                                                                    end: 634,
                                                                                    as_str(): "4u8",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 622,
                                                                    end: 634,
                                                                    as_str(): "third == 4u8",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 574,
                                                        end: 634,
                                                        as_str(): "third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8",
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
                                                                                Boolean(
                                                                                    false,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 641,
                                                                                end: 646,
                                                                                as_str(): "false",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 641,
                                                                        end: 646,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 635,
                                                                end: 650,
                                                                as_str(): "{\n    false\n  }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 635,
                                                        end: 650,
                                                        as_str(): "{\n    false\n  }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: If(
                                                            IfExpression {
                                                                condition: Expression {
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
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 665,
                                                                                                    end: 667,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 665,
                                                                                                    end: 667,
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
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 665,
                                                                                                end: 667,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 665,
                                                                                    end: 667,
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
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 659,
                                                                                                end: 664,
                                                                                                as_str(): "third",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 659,
                                                                                        end: 664,
                                                                                        as_str(): "third",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        U8(
                                                                                            5,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 668,
                                                                                        end: 671,
                                                                                        as_str(): "5u8",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 659,
                                                                        end: 671,
                                                                        as_str(): "third == 5u8",
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
                                                                                                Boolean(
                                                                                                    true,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 678,
                                                                                                end: 682,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 678,
                                                                                        end: 682,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 672,
                                                                                end: 686,
                                                                                as_str(): "{\n    true\n  }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 672,
                                                                        end: 686,
                                                                        as_str(): "{\n    true\n  }",
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
                                                                                                    Boolean(
                                                                                                        false,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 698,
                                                                                                    end: 703,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 698,
                                                                                            end: 703,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 692,
                                                                                    end: 707,
                                                                                    as_str(): "{\n    false\n  }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 692,
                                                                            end: 707,
                                                                            as_str(): "{\n    false\n  }",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 707,
                                                            as_str(): "if third == 5u8 {\n    true\n  } else {\n    false\n  }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 571,
                                            end: 707,
                                            as_str(): "if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 571,
                                    end: 707,
                                    as_str(): "if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 536,
                            end: 709,
                            as_str(): "{\n  let third = third_if(value);\n  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 518,
                                    end: 523,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 525,
                                        end: 526,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 525,
                                end: 526,
                                as_str(): "E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 502,
                        end: 709,
                        as_str(): "fn second_if<E>(value: E) -> bool {\n  let third = third_if(value);\n  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [
                        E: TypeId(31632),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 531,
                        end: 535,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 502,
            end: 709,
            as_str(): "fn second_if<E>(value: E) -> bool {\n  let third = third_if(value);\n  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 714,
                            end: 722,
                            as_str(): "first_if",
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 751,
                                                    end: 757,
                                                    as_str(): "second",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 760,
                                                                        end: 769,
                                                                        as_str(): "second_if",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 760,
                                                                end: 769,
                                                                as_str(): "second_if",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 770,
                                                                            end: 775,
                                                                            as_str(): "value",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 770,
                                                                    end: 775,
                                                                    as_str(): "value",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 760,
                                                    end: 776,
                                                    as_str(): "second_if(value)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 747,
                                    end: 777,
                                    as_str(): "let second = second_if(value);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 790,
                                                                                    end: 792,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 790,
                                                                                    end: 792,
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 790,
                                                                                end: 792,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 790,
                                                                    end: 792,
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 783,
                                                                                end: 789,
                                                                                as_str(): "second",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 783,
                                                                        end: 789,
                                                                        as_str(): "second",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Boolean(
                                                                            false,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 793,
                                                                        end: 798,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 783,
                                                        end: 798,
                                                        as_str(): "second == false",
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
                                                                                U64(
                                                                                    2,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 805,
                                                                                end: 809,
                                                                                as_str(): "2u64",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 805,
                                                                        end: 809,
                                                                        as_str(): "2u64",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 799,
                                                                end: 813,
                                                                as_str(): "{\n    2u64\n  }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 799,
                                                        end: 813,
                                                        as_str(): "{\n    2u64\n  }",
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
                                                                                    U64(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 825,
                                                                                    end: 829,
                                                                                    as_str(): "3u64",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 825,
                                                                            end: 829,
                                                                            as_str(): "3u64",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 819,
                                                                    end: 833,
                                                                    as_str(): "{\n    3u64\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 819,
                                                            end: 833,
                                                            as_str(): "{\n    3u64\n  }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 780,
                                            end: 833,
                                            as_str(): "if second == false {\n    2u64\n  } else {\n    3u64\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 780,
                                    end: 833,
                                    as_str(): "if second == false {\n    2u64\n  } else {\n    3u64\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 743,
                            end: 835,
                            as_str(): "{\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 726,
                                    end: 731,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 733,
                                        end: 734,
                                        as_str(): "F",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 733,
                                end: 734,
                                as_str(): "F",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 711,
                        end: 835,
                        as_str(): "fn first_if<F>(value: F) -> u64 {\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [
                        F: TypeId(31633),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 739,
                        end: 742,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 711,
            end: 835,
            as_str(): "fn first_if<F>(value: F) -> u64 {\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 840,
                            end: 853,
                            as_str(): "double_double",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 890,
                                                    end: 896,
                                                    as_str(): "second",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 890,
                                            end: 896,
                                            as_str(): "second",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 890,
                                    end: 896,
                                    as_str(): "second",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 886,
                            end: 898,
                            as_str(): "{\n  second\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 860,
                                    end: 865,
                                    as_str(): "first",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 867,
                                        end: 868,
                                        as_str(): "Y",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 867,
                                end: 868,
                                as_str(): "Y",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 870,
                                    end: 876,
                                    as_str(): "second",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 878,
                                        end: 879,
                                        as_str(): "Z",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 878,
                                end: 879,
                                as_str(): "Z",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 837,
                        end: 898,
                        as_str(): "fn double_double<Y, Z>(first: Y, second: Z) -> Z {\n  second\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 884,
                                end: 885,
                                as_str(): "Z",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        Y: TypeId(31634),
                        Z: TypeId(31635),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 884,
                        end: 885,
                        as_str(): "Z",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 837,
            end: 898,
            as_str(): "fn double_double<Y, Z>(first: Y, second: Z) -> Z {\n  second\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 903,
                            end: 909,
                            as_str(): "double",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 937,
                                                                end: 950,
                                                                as_str(): "double_double",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 937,
                                                        end: 950,
                                                        as_str(): "double_double",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                false,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 951,
                                                            end: 956,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 958,
                                                                    end: 968,
                                                                    as_str(): "the_second",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 958,
                                                            end: 968,
                                                            as_str(): "the_second",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 937,
                                            end: 969,
                                            as_str(): "double_double(false, the_second)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 937,
                                    end: 969,
                                    as_str(): "double_double(false, the_second)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 933,
                            end: 971,
                            as_str(): "{\n  double_double(false, the_second)\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 913,
                                    end: 923,
                                    as_str(): "the_second",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 925,
                                        end: 926,
                                        as_str(): "Y",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 925,
                                end: 926,
                                as_str(): "Y",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 900,
                        end: 971,
                        as_str(): "fn double<Y>(the_second: Y) -> Y {\n  double_double(false, the_second)\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 931,
                                end: 932,
                                as_str(): "Y",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        Y: TypeId(31636),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 931,
                        end: 932,
                        as_str(): "Y",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 900,
            end: 971,
            as_str(): "fn double<Y>(the_second: Y) -> Y {\n  double_double(false, the_second)\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 976,
                            end: 989,
                            as_str(): "generic_match",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                            "__match_return_var_name_4",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1020,
                                                                            end: 1025,
                                                                            as_str(): "value",
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1020,
                                                                                    end: 1025,
                                                                                    as_str(): "value",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1020,
                                                                            end: 1025,
                                                                            as_str(): "value",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1014,
                                                            end: 1048,
                                                            as_str(): "match value {\n    foo => 3u64,\n  }",
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1020,
                                                                                        end: 1025,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 1020,
                                                                                end: 1025,
                                                                                as_str(): "value",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Variable {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1032,
                                                                                            end: 1035,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1032,
                                                                                        end: 1035,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        U64(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1039,
                                                                                        end: 1043,
                                                                                        as_str(): "3u64",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1032,
                                                                                    end: 1044,
                                                                                    as_str(): "foo => 3u64,",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1014,
                                                                    end: 1048,
                                                                    as_str(): "match value {\n    foo => 3u64,\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1014,
                                                            end: 1048,
                                                            as_str(): "match value {\n    foo => 3u64,\n  }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1014,
                                                    end: 1048,
                                                    as_str(): "match value {\n    foo => 3u64,\n  }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1014,
                                            end: 1048,
                                            as_str(): "match value {\n    foo => 3u64,\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1014,
                                    end: 1048,
                                    as_str(): "match value {\n    foo => 3u64,\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1010,
                            end: 1050,
                            as_str(): "{\n  match value {\n    foo => 3u64,\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 993,
                                    end: 998,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1000,
                                        end: 1001,
                                        as_str(): "G",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 1000,
                                end: 1001,
                                as_str(): "G",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 973,
                        end: 1050,
                        as_str(): "fn generic_match<G>(value: G) -> u64 {\n  match value {\n    foo => 3u64,\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [
                        G: TypeId(31637),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 1006,
                        end: 1009,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 973,
            end: 1050,
            as_str(): "fn generic_match<G>(value: G) -> u64 {\n  match value {\n    foo => 3u64,\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1055,
                            end: 1065,
                            as_str(): "generic_if",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1093,
                                                        end: 1097,
                                                        as_str(): "true",
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
                                                                                U64(
                                                                                    3,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 1104,
                                                                                end: 1108,
                                                                                as_str(): "3u64",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1104,
                                                                        end: 1108,
                                                                        as_str(): "3u64",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1098,
                                                                end: 1112,
                                                                as_str(): "{\n    3u64\n  }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1098,
                                                        end: 1112,
                                                        as_str(): "{\n    3u64\n  }",
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
                                                                                    U64(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1124,
                                                                                    end: 1128,
                                                                                    as_str(): "1u64",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1124,
                                                                            end: 1128,
                                                                            as_str(): "1u64",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1118,
                                                                    end: 1132,
                                                                    as_str(): "{\n    1u64\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1118,
                                                            end: 1132,
                                                            as_str(): "{\n    1u64\n  }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1090,
                                            end: 1132,
                                            as_str(): "if true {\n    3u64\n  } else {\n    1u64\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1090,
                                    end: 1132,
                                    as_str(): "if true {\n    3u64\n  } else {\n    1u64\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1086,
                            end: 1134,
                            as_str(): "{\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1069,
                                    end: 1074,
                                    as_str(): "value",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1076,
                                        end: 1077,
                                        as_str(): "H",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe09f0b11e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                ),
                                start: 1076,
                                end: 1077,
                                as_str(): "H",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 1052,
                        end: 1134,
                        as_str(): "fn generic_if<H>(value: H) -> u64 {\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [
                        H: TypeId(31638),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 1082,
                        end: 1085,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 1052,
            end: 1134,
            as_str(): "fn generic_if<H>(value: H) -> u64 {\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
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
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1139,
                            end: 1143,
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
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1161,
                                                    end: 1162,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1165,
                                                                        end: 1176,
                                                                        as_str(): "first_match",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1165,
                                                                end: 1176,
                                                                as_str(): "first_match",
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1177,
                                                                    end: 1181,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1165,
                                                    end: 1182,
                                                    as_str(): "first_match(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1157,
                                    end: 1183,
                                    as_str(): "let a = first_match(true);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1186,
                                                                end: 1192,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1186,
                                                        end: 1192,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1195,
                                                                                        end: 1197,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1195,
                                                                                        end: 1197,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1195,
                                                                                    end: 1197,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1195,
                                                                        end: 1197,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1193,
                                                                                    end: 1194,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1193,
                                                                            end: 1194,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1198,
                                                                            end: 1199,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1193,
                                                            end: 1199,
                                                            as_str(): "a == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1186,
                                            end: 1200,
                                            as_str(): "assert(a == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1186,
                                    end: 1200,
                                    as_str(): "assert(a == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1209,
                                                    end: 1210,
                                                    as_str(): "b",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1213,
                                                                        end: 1224,
                                                                        as_str(): "first_match",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1213,
                                                                end: 1224,
                                                                as_str(): "first_match",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1225,
                                                                    end: 1228,
                                                                    as_str(): "1u8",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1213,
                                                    end: 1229,
                                                    as_str(): "first_match(1u8)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1205,
                                    end: 1230,
                                    as_str(): "let b = first_match(1u8);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1233,
                                                                end: 1239,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1233,
                                                        end: 1239,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1242,
                                                                                        end: 1244,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1242,
                                                                                        end: 1244,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1242,
                                                                                    end: 1244,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1242,
                                                                        end: 1244,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1240,
                                                                                    end: 1241,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1240,
                                                                            end: 1241,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1245,
                                                                            end: 1246,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1240,
                                                            end: 1246,
                                                            as_str(): "b == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1233,
                                            end: 1247,
                                            as_str(): "assert(b == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1233,
                                    end: 1247,
                                    as_str(): "assert(b == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1256,
                                                    end: 1257,
                                                    as_str(): "c",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1260,
                                                                        end: 1268,
                                                                        as_str(): "first_if",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1260,
                                                                end: 1268,
                                                                as_str(): "first_if",
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1269,
                                                                    end: 1273,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1260,
                                                    end: 1274,
                                                    as_str(): "first_if(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1252,
                                    end: 1275,
                                    as_str(): "let c = first_if(true);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1278,
                                                                end: 1284,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1278,
                                                        end: 1284,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1287,
                                                                                        end: 1289,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1287,
                                                                                        end: 1289,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1287,
                                                                                    end: 1289,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1287,
                                                                        end: 1289,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1285,
                                                                                    end: 1286,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1285,
                                                                            end: 1286,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1290,
                                                                            end: 1291,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1285,
                                                            end: 1291,
                                                            as_str(): "c == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1278,
                                            end: 1292,
                                            as_str(): "assert(c == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1278,
                                    end: 1292,
                                    as_str(): "assert(c == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1301,
                                                    end: 1302,
                                                    as_str(): "d",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1305,
                                                                        end: 1313,
                                                                        as_str(): "first_if",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1305,
                                                                end: 1313,
                                                                as_str(): "first_if",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1314,
                                                                    end: 1317,
                                                                    as_str(): "1u8",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1305,
                                                    end: 1318,
                                                    as_str(): "first_if(1u8)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1297,
                                    end: 1319,
                                    as_str(): "let d = first_if(1u8);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1322,
                                                                end: 1328,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1322,
                                                        end: 1328,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1331,
                                                                                        end: 1333,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1331,
                                                                                        end: 1333,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1331,
                                                                                    end: 1333,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1331,
                                                                        end: 1333,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1329,
                                                                                    end: 1330,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1329,
                                                                            end: 1330,
                                                                            as_str(): "d",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1334,
                                                                            end: 1335,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1329,
                                                            end: 1335,
                                                            as_str(): "d == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1322,
                                            end: 1336,
                                            as_str(): "assert(d == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1322,
                                    end: 1336,
                                    as_str(): "assert(d == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1345,
                                                    end: 1346,
                                                    as_str(): "e",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1349,
                                                                        end: 1362,
                                                                        as_str(): "generic_match",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1349,
                                                                end: 1362,
                                                                as_str(): "generic_match",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        6,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1363,
                                                                    end: 1364,
                                                                    as_str(): "6",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1349,
                                                    end: 1365,
                                                    as_str(): "generic_match(6)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1341,
                                    end: 1366,
                                    as_str(): "let e = generic_match(6);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1369,
                                                                end: 1375,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1369,
                                                        end: 1375,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1378,
                                                                                        end: 1380,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1378,
                                                                                        end: 1380,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1378,
                                                                                    end: 1380,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1378,
                                                                        end: 1380,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1376,
                                                                                    end: 1377,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1376,
                                                                            end: 1377,
                                                                            as_str(): "e",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1381,
                                                                            end: 1382,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1376,
                                                            end: 1382,
                                                            as_str(): "e == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1369,
                                            end: 1383,
                                            as_str(): "assert(e == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1369,
                                    end: 1383,
                                    as_str(): "assert(e == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1392,
                                                    end: 1393,
                                                    as_str(): "f",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1396,
                                                                        end: 1409,
                                                                        as_str(): "generic_match",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1396,
                                                                end: 1409,
                                                                as_str(): "generic_match",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        false,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1410,
                                                                    end: 1415,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1396,
                                                    end: 1416,
                                                    as_str(): "generic_match(false)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1388,
                                    end: 1417,
                                    as_str(): "let f = generic_match(false);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1420,
                                                                end: 1426,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1420,
                                                        end: 1426,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1429,
                                                                                        end: 1431,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1429,
                                                                                        end: 1431,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1429,
                                                                                    end: 1431,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1429,
                                                                        end: 1431,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1427,
                                                                                    end: 1428,
                                                                                    as_str(): "f",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1427,
                                                                            end: 1428,
                                                                            as_str(): "f",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1432,
                                                                            end: 1433,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1427,
                                                            end: 1433,
                                                            as_str(): "f == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1420,
                                            end: 1434,
                                            as_str(): "assert(f == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1420,
                                    end: 1434,
                                    as_str(): "assert(f == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1443,
                                                    end: 1444,
                                                    as_str(): "g",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1447,
                                                                        end: 1457,
                                                                        as_str(): "generic_if",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1447,
                                                                end: 1457,
                                                                as_str(): "generic_if",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        6,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1458,
                                                                    end: 1459,
                                                                    as_str(): "6",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1447,
                                                    end: 1460,
                                                    as_str(): "generic_if(6)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1439,
                                    end: 1461,
                                    as_str(): "let g = generic_if(6);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1464,
                                                                end: 1470,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1464,
                                                        end: 1470,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1473,
                                                                                        end: 1475,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1473,
                                                                                        end: 1475,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1473,
                                                                                    end: 1475,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1473,
                                                                        end: 1475,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1471,
                                                                                    end: 1472,
                                                                                    as_str(): "g",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1471,
                                                                            end: 1472,
                                                                            as_str(): "g",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1476,
                                                                            end: 1477,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1471,
                                                            end: 1477,
                                                            as_str(): "g == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1464,
                                            end: 1478,
                                            as_str(): "assert(g == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1464,
                                    end: 1478,
                                    as_str(): "assert(g == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1487,
                                                    end: 1488,
                                                    as_str(): "h",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1491,
                                                                        end: 1501,
                                                                        as_str(): "generic_if",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1491,
                                                                end: 1501,
                                                                as_str(): "generic_if",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        false,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1502,
                                                                    end: 1507,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1491,
                                                    end: 1508,
                                                    as_str(): "generic_if(false)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1483,
                                    end: 1509,
                                    as_str(): "let h = generic_if(false);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1512,
                                                                end: 1518,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1512,
                                                        end: 1518,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1521,
                                                                                        end: 1523,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1521,
                                                                                        end: 1523,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1521,
                                                                                    end: 1523,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1521,
                                                                        end: 1523,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1519,
                                                                                    end: 1520,
                                                                                    as_str(): "h",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1519,
                                                                            end: 1520,
                                                                            as_str(): "h",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1524,
                                                                            end: 1525,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1519,
                                                            end: 1525,
                                                            as_str(): "h == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1512,
                                            end: 1526,
                                            as_str(): "assert(h == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1512,
                                    end: 1526,
                                    as_str(): "assert(h == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1535,
                                                    end: 1536,
                                                    as_str(): "i",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1539,
                                                                        end: 1545,
                                                                        as_str(): "double",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1539,
                                                                end: 1545,
                                                                as_str(): "double",
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1546,
                                                                    end: 1551,
                                                                    as_str(): "10u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1539,
                                                    end: 1552,
                                                    as_str(): "double(10u32)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1531,
                                    end: 1553,
                                    as_str(): "let i = double(10u32);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1556,
                                                                end: 1562,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1556,
                                                        end: 1562,
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1565,
                                                                                        end: 1567,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1565,
                                                                                        end: 1567,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1565,
                                                                                    end: 1567,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1565,
                                                                        end: 1567,
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1563,
                                                                                    end: 1564,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1563,
                                                                            end: 1564,
                                                                            as_str(): "i",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                10,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1568,
                                                                            end: 1573,
                                                                            as_str(): "10u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1563,
                                                            end: 1573,
                                                            as_str(): "i == 10u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1556,
                                            end: 1574,
                                            as_str(): "assert(i == 10u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1556,
                                    end: 1574,
                                    as_str(): "assert(i == 10u32)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1583,
                                                    end: 1584,
                                                    as_str(): "j",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1587,
                                                                        end: 1593,
                                                                        as_str(): "double",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1587,
                                                                end: 1593,
                                                                as_str(): "double",
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1594,
                                                                    end: 1598,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1587,
                                                    end: 1599,
                                                    as_str(): "double(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1579,
                                    end: 1600,
                                    as_str(): "let j = double(true);",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1603,
                                                                end: 1609,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1603,
                                                        end: 1609,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1610,
                                                                    end: 1611,
                                                                    as_str(): "j",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1610,
                                                            end: 1611,
                                                            as_str(): "j",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1603,
                                            end: 1612,
                                            as_str(): "assert(j)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1603,
                                    end: 1612,
                                    as_str(): "assert(j)",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1617,
                                            end: 1618,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09f0b11e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                    ),
                                    start: 1617,
                                    end: 1618,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 1153,
                            end: 1620,
                            as_str(): "{\n  let a = first_match(true);\n  assert(a == 3);\n\n  let b = first_match(1u8);\n  assert(b == 3);\n\n  let c = first_if(true);\n  assert(c == 3);\n\n  let d = first_if(1u8);\n  assert(d == 3);\n\n  let e = generic_match(6);\n  assert(e == 3);\n\n  let f = generic_match(false);\n  assert(f == 3);\n\n  let g = generic_if(6);\n  assert(g == 3);\n\n  let h = generic_if(false);\n  assert(h == 3);\n\n  let i = double(10u32);\n  assert(i == 10u32);\n\n  let j = double(true);\n  assert(j);\n\n  1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 1136,
                        end: 1620,
                        as_str(): "fn main() -> u64 {\n  let a = first_match(true);\n  assert(a == 3);\n\n  let b = first_match(1u8);\n  assert(b == 3);\n\n  let c = first_if(true);\n  assert(c == 3);\n\n  let d = first_if(1u8);\n  assert(d == 3);\n\n  let e = generic_match(6);\n  assert(e == 3);\n\n  let f = generic_match(false);\n  assert(f == 3);\n\n  let g = generic_if(6);\n  assert(g == 3);\n\n  let h = generic_if(false);\n  assert(h == 3);\n\n  let i = double(10u32);\n  assert(i == 10u32);\n\n  let j = double(true);\n  assert(j);\n\n  1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                        ),
                        start: 1149,
                        end: 1152,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe09f0b11e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
            ),
            start: 1136,
            end: 1620,
            as_str(): "fn main() -> u64 {\n  let a = first_match(true);\n  assert(a == 3);\n\n  let b = first_match(1u8);\n  assert(b == 3);\n\n  let c = first_if(true);\n  assert(c == 3);\n\n  let d = first_if(1u8);\n  assert(d == 3);\n\n  let e = generic_match(6);\n  assert(e == 3);\n\n  let f = generic_match(false);\n  assert(f == 3);\n\n  let g = generic_if(6);\n  assert(g == 3);\n\n  let h = generic_if(false);\n  assert(h == 3);\n\n  let i = double(10u32);\n  assert(i == 10u32);\n\n  let j = double(true);\n  assert(j);\n\n  1\n}",
        },
    },
]
