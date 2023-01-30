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
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 12,
                            end: 17,
                            as_str(): "abort",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Asm(
                                            AsmExpression {
                                                registers: [],
                                                body: [],
                                                returns: Some(
                                                    (
                                                        AsmRegister {
                                                            name: "one",
                                                        },
                                                        Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 50,
                                                            end: 53,
                                                            as_str(): "one",
                                                        },
                                                    ),
                                                ),
                                                return_type: Boolean,
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 34,
                                                    end: 77,
                                                    as_str(): "asm() {\n        one: bool // Failure.\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 34,
                                            end: 77,
                                            as_str(): "asm() {\n        one: bool // Failure.\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13ad4b060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                    ),
                                    start: 34,
                                    end: 77,
                                    as_str(): "asm() {\n        one: bool // Failure.\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 28,
                            end: 79,
                            as_str(): "{\n    asm() {\n        one: bool // Failure.\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb13ad4b060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                        ),
                        start: 9,
                        end: 79,
                        as_str(): "fn abort() -> bool {\n    asm() {\n        one: bool // Failure.\n    }\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb13ad4b060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                        ),
                        start: 23,
                        end: 27,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13ad4b060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
            ),
            start: 9,
            end: 79,
            as_str(): "fn abort() -> bool {\n    asm() {\n        one: bool // Failure.\n    }\n}",
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
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 84,
                            end: 88,
                            as_str(): "main",
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
                                                    kind: LazyOperator(
                                                        LazyOperatorExpression {
                                                            op: And,
                                                            lhs: Expression {
                                                                kind: LazyOperator(
                                                                    LazyOperatorExpression {
                                                                        op: And,
                                                                        lhs: Expression {
                                                                            kind: Literal(
                                                                                Boolean(
                                                                                    true,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 108,
                                                                                end: 112,
                                                                                as_str(): "true",
                                                                            },
                                                                        },
                                                                        rhs: Expression {
                                                                            kind: Literal(
                                                                                Boolean(
                                                                                    false,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 116,
                                                                                end: 121,
                                                                                as_str(): "false",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 108,
                                                                    end: 121,
                                                                    as_str(): "true && false",
                                                                },
                                                            },
                                                            rhs: Expression {
                                                                kind: FunctionApplication(
                                                                    FunctionApplicationExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 126,
                                                                                        end: 131,
                                                                                        as_str(): "abort",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 126,
                                                                                end: 131,
                                                                                as_str(): "abort",
                                                                            },
                                                                        },
                                                                        arguments: [],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 133,
                                                                    as_str(): "abort()",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb13ad4b060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 133,
                                                        as_str(): "(true && false) && abort()",
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
                                                                                    2,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 165,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                        ),
                                                                        start: 164,
                                                                        end: 165,
                                                                        as_str(): "2",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 171,
                                                                as_str(): "{\n        // Failure.\n        2\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb13ad4b060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                        ),
                                                        start: 134,
                                                        end: 171,
                                                        as_str(): "{\n        // Failure.\n        2\n    }",
                                                    },
                                                },
                                                else: Some(
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
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    false,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 181,
                                                                                                end: 186,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                        },
                                                                                        rhs: Expression {
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    true,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 190,
                                                                                                end: 194,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 181,
                                                                                    end: 194,
                                                                                    as_str(): "false || true",
                                                                                },
                                                                            },
                                                                            rhs: Expression {
                                                                                kind: FunctionApplication(
                                                                                    FunctionApplicationExpression {
                                                                                        call_path_binding: TypeBinding {
                                                                                            inner: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                        ),
                                                                                                        start: 199,
                                                                                                        end: 204,
                                                                                                        as_str(): "abort",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 199,
                                                                                                end: 204,
                                                                                                as_str(): "abort",
                                                                                            },
                                                                                        },
                                                                                        arguments: [],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 199,
                                                                                    end: 206,
                                                                                    as_str(): "abort()",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                        ),
                                                                        start: 180,
                                                                        end: 206,
                                                                        as_str(): "(false || true) || abort()",
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
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 237,
                                                                                                end: 239,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 237,
                                                                                        end: 239,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 207,
                                                                                end: 245,
                                                                                as_str(): "{\n        // Success.\n        42\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                        ),
                                                                        start: 207,
                                                                        end: 245,
                                                                        as_str(): "{\n        // Success.\n        42\n    }",
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
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                    ),
                                                                                                    start: 281,
                                                                                                    end: 282,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                            ),
                                                                                            start: 281,
                                                                                            end: 282,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 251,
                                                                                    end: 288,
                                                                                    as_str(): "{\n        // Failure.\n        3\n    }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                            ),
                                                                            start: 251,
                                                                            end: 288,
                                                                            as_str(): "{\n        // Failure.\n        3\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 177,
                                                            end: 288,
                                                            as_str(): "if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 104,
                                            end: 288,
                                            as_str(): "if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13ad4b060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                    ),
                                    start: 104,
                                    end: 288,
                                    as_str(): "if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 98,
                            end: 290,
                            as_str(): "{\n    if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb13ad4b060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                        ),
                        start: 81,
                        end: 290,
                        as_str(): "fn main() -> u64 {\n    if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb13ad4b060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                        ),
                        start: 94,
                        end: 97,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13ad4b060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
            ),
            start: 81,
            end: 290,
            as_str(): "fn main() -> u64 {\n    if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }\n}",
        },
    },
]
