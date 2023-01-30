[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe033497b40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                    ),
                    start: 8,
                    end: 22,
                    as_str(): "dep shiftable;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe033497b40,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                    ),
                    start: 12,
                    end: 21,
                    as_str(): "shiftable",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe033497b40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
            ),
            start: 8,
            end: 22,
            as_str(): "dep shiftable;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe033497b40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                            ),
                            start: 27,
                            end: 36,
                            as_str(): "shiftable",
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
            src (ptr): 0x00007fe033497b40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
            ),
            start: 23,
            end: 40,
            as_str(): "use shiftable::*;",
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
                            src (ptr): 0x00007fe033497b40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                            ),
                            start: 45,
                            end: 49,
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
                                                    src (ptr): 0x00007fe033497b40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 77,
                                                    as_str(): "shiftAnswer",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe033497b40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                    ),
                                                    start: 79,
                                                    end: 82,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe033497b40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                    ),
                                                    start: 85,
                                                    end: 86,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe033497b40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                    ),
                                    start: 58,
                                    end: 87,
                                    as_str(): "let mut shiftAnswer: u64 = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe033497b40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 108,
                                                                as_str(): "rsh",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe033497b40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                        ),
                                                        start: 105,
                                                        end: 108,
                                                        as_str(): "rsh",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe033497b40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                                    ),
                                                                    start: 93,
                                                                    end: 104,
                                                                    as_str(): "shiftAnswer",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 93,
                                                            end: 104,
                                                            as_str(): "shiftAnswer",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                5,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe033497b40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 110,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe033497b40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                            ),
                                            start: 93,
                                            end: 111,
                                            as_str(): "shiftAnswer.rsh(5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe033497b40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                                    ),
                                    start: 93,
                                    end: 111,
                                    as_str(): "shiftAnswer.rsh(5)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe033497b40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                            ),
                            start: 52,
                            end: 114,
                            as_str(): "{\n    let mut shiftAnswer: u64 = 0;\n\n    shiftAnswer.rsh(5);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe033497b40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                        ),
                        start: 42,
                        end: 114,
                        as_str(): "fn main() {\n    let mut shiftAnswer: u64 = 0;\n\n    shiftAnswer.rsh(5);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe033497b40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
                        ),
                        start: 42,
                        end: 51,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe033497b40,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqNmksI/trait_import_with_star/src/main.sw",
            ),
            start: 42,
            end: 114,
            as_str(): "fn main() {\n    let mut shiftAnswer: u64 = 0;\n\n    shiftAnswer.rsh(5);\n}",
        },
    },
]
