[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
            src (ptr): 0x00007f8a0e6287a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
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
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 38,
                            end: 42,
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
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 67,
                                                    end: 74,
                                                    as_str(): "counter",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 78,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 79,
                                    as_str(): "let mut counter = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 131,
                                                                                    end: 132,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 131,
                                                                                    end: 132,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "lt",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 131,
                                                                                end: 132,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 131,
                                                                    end: 132,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 123,
                                                                                end: 130,
                                                                                as_str(): "counter",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 123,
                                                                        end: 130,
                                                                        as_str(): "counter",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 133,
                                                                        end: 135,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 123,
                                                        end: 135,
                                                        as_str(): "counter < 10",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: Reassignment(
                                                                        ReassignmentExpression {
                                                                            lhs: VariableExpression(
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 146,
                                                                                                end: 153,
                                                                                                as_str(): "counter",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 146,
                                                                                        end: 153,
                                                                                        as_str(): "counter",
                                                                                    },
                                                                                },
                                                                            ),
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 164,
                                                                                                                end: 165,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 164,
                                                                                                                end: 165,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 164,
                                                                                                            end: 165,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 164,
                                                                                                end: 165,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 156,
                                                                                                            end: 163,
                                                                                                            as_str(): "counter",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 156,
                                                                                                    end: 163,
                                                                                                    as_str(): "counter",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 166,
                                                                                                    end: 167,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 156,
                                                                                    end: 167,
                                                                                    as_str(): "counter + 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 167,
                                                                        as_str(): "counter = counter + 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 146,
                                                                end: 167,
                                                                as_str(): "counter = counter + 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 136,
                                                        end: 174,
                                                        as_str(): "{\n        counter = counter + 1;\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 174,
                                            as_str(): "while counter < 10 {\n        counter = counter + 1;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 117,
                                    end: 174,
                                    as_str(): "while counter < 10 {\n        counter = counter + 1;\n    }",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 185,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 179,
                                                        end: 185,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 196,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 196,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 194,
                                                                                    end: 196,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 196,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 186,
                                                                                    end: 193,
                                                                                    as_str(): "counter",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 186,
                                                                            end: 193,
                                                                            as_str(): "counter",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                10,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 199,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 199,
                                                            as_str(): "counter == 10",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 179,
                                            end: 200,
                                            as_str(): "assert(counter == 10)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 179,
                                    end: 200,
                                    as_str(): "assert(counter == 10)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 301,
                                                    end: 310,
                                                    as_str(): "counter_2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 313,
                                                    end: 314,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 293,
                                    end: 315,
                                    as_str(): "let mut counter_2 = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 328,
                                                    end: 337,
                                                    as_str(): "counter_3",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 340,
                                                    end: 341,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 320,
                                    end: 342,
                                    as_str(): "let mut counter_3 = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 363,
                                                                                    end: 364,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 363,
                                                                                    end: 364,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "lt",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 363,
                                                                                end: 364,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 363,
                                                                    end: 364,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 353,
                                                                                end: 362,
                                                                                as_str(): "counter_2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 353,
                                                                        end: 362,
                                                                        as_str(): "counter_2",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 365,
                                                                        end: 367,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 353,
                                                        end: 367,
                                                        as_str(): "counter_2 < 10",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 391,
                                                                                                                end: 393,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 391,
                                                                                                                end: 393,
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
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 391,
                                                                                                            end: 393,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 391,
                                                                                                end: 393,
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
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 381,
                                                                                                            end: 390,
                                                                                                            as_str(): "counter_2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 381,
                                                                                                    end: 390,
                                                                                                    as_str(): "counter_2",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 394,
                                                                                                    end: 395,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 381,
                                                                                    end: 395,
                                                                                    as_str(): "counter_2 == 3",
                                                                                },
                                                                            },
                                                                            then: Expression {
                                                                                kind: CodeBlock(
                                                                                    CodeBlock {
                                                                                        contents: [
                                                                                            AstNode {
                                                                                                content: Expression(
                                                                                                    Expression {
                                                                                                        kind: Reassignment(
                                                                                                            ReassignmentExpression {
                                                                                                                lhs: VariableExpression(
                                                                                                                    Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 463,
                                                                                                                                    end: 472,
                                                                                                                                    as_str(): "counter_2",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 463,
                                                                                                                            end: 472,
                                                                                                                            as_str(): "counter_2",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                rhs: Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            10,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 475,
                                                                                                                        end: 477,
                                                                                                                        as_str(): "10",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 463,
                                                                                                            end: 477,
                                                                                                            as_str(): "counter_2 = 10",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 463,
                                                                                                    end: 477,
                                                                                                    as_str(): "counter_2 = 10",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 396,
                                                                                            end: 488,
                                                                                            as_str(): "{\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 396,
                                                                                    end: 488,
                                                                                    as_str(): "{\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        }",
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
                                                                                                            kind: Reassignment(
                                                                                                                ReassignmentExpression {
                                                                                                                    lhs: VariableExpression(
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 508,
                                                                                                                                        end: 517,
                                                                                                                                        as_str(): "counter_2",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 508,
                                                                                                                                end: 517,
                                                                                                                                as_str(): "counter_2",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
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
                                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 530,
                                                                                                                                                        end: 531,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "ops",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 530,
                                                                                                                                                        end: 531,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "add",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 530,
                                                                                                                                                    end: 531,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            is_absolute: true,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 530,
                                                                                                                                        end: 531,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 520,
                                                                                                                                                    end: 529,
                                                                                                                                                    as_str(): "counter_2",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 520,
                                                                                                                                            end: 529,
                                                                                                                                            as_str(): "counter_2",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Literal(
                                                                                                                                            Numeric(
                                                                                                                                                1,
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 532,
                                                                                                                                            end: 533,
                                                                                                                                            as_str(): "1",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 520,
                                                                                                                            end: 533,
                                                                                                                            as_str(): "counter_2 + 1",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 508,
                                                                                                                end: 533,
                                                                                                                as_str(): "counter_2 = counter_2 + 1",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 508,
                                                                                                        end: 533,
                                                                                                        as_str(): "counter_2 = counter_2 + 1",
                                                                                                    },
                                                                                                },
                                                                                                AstNode {
                                                                                                    content: Expression(
                                                                                                        Expression {
                                                                                                            kind: Reassignment(
                                                                                                                ReassignmentExpression {
                                                                                                                    lhs: VariableExpression(
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 547,
                                                                                                                                        end: 556,
                                                                                                                                        as_str(): "counter_3",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 547,
                                                                                                                                end: 556,
                                                                                                                                as_str(): "counter_3",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
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
                                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 569,
                                                                                                                                                        end: 570,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "ops",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 569,
                                                                                                                                                        end: 570,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "add",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 569,
                                                                                                                                                    end: 570,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            is_absolute: true,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 569,
                                                                                                                                        end: 570,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 559,
                                                                                                                                                    end: 568,
                                                                                                                                                    as_str(): "counter_3",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 559,
                                                                                                                                            end: 568,
                                                                                                                                            as_str(): "counter_3",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Literal(
                                                                                                                                            Numeric(
                                                                                                                                                1,
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 571,
                                                                                                                                            end: 572,
                                                                                                                                            as_str(): "1",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 559,
                                                                                                                            end: 572,
                                                                                                                            as_str(): "counter_3 + 1",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 547,
                                                                                                                end: 572,
                                                                                                                as_str(): "counter_3 = counter_3 + 1",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 547,
                                                                                                        end: 572,
                                                                                                        as_str(): "counter_3 = counter_3 + 1",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 494,
                                                                                                end: 583,
                                                                                                as_str(): "{\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 494,
                                                                                        end: 583,
                                                                                        as_str(): "{\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 378,
                                                                        end: 583,
                                                                        as_str(): "if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 378,
                                                                end: 583,
                                                                as_str(): "if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 368,
                                                        end: 589,
                                                        as_str(): "{\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 589,
                                            as_str(): "while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 589,
                                    as_str(): "while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 595,
                                                                end: 601,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 595,
                                                        end: 601,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
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
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 602,
                                                                                                end: 611,
                                                                                                as_str(): "counter_2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 602,
                                                                                        end: 611,
                                                                                        as_str(): "counter_2",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            10,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 615,
                                                                                        end: 617,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 602,
                                                                        end: 617,
                                                                        as_str(): "counter_2 == 10",
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
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 631,
                                                                                                    end: 633,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 631,
                                                                                                    end: 633,
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
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 631,
                                                                                                end: 633,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 631,
                                                                                    end: 633,
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
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 621,
                                                                                                end: 630,
                                                                                                as_str(): "counter_3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 621,
                                                                                        end: 630,
                                                                                        as_str(): "counter_3",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 634,
                                                                                        end: 635,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 621,
                                                                        end: 635,
                                                                        as_str(): "counter_3 == 3",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 602,
                                                            end: 635,
                                                            as_str(): "counter_2 == 10 && counter_3 == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 595,
                                            end: 636,
                                            as_str(): "assert(counter_2 == 10 && counter_3 == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 595,
                                    end: 636,
                                    as_str(): "assert(counter_2 == 10 && counter_3 == 3)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 677,
                                                    end: 686,
                                                    as_str(): "counter_4",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 689,
                                                    end: 690,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 669,
                                    end: 691,
                                    as_str(): "let mut counter_4 = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 704,
                                                    end: 713,
                                                    as_str(): "counter_5",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 716,
                                                    end: 717,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 696,
                                    end: 718,
                                    as_str(): "let mut counter_5 = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 740,
                                                                                    end: 741,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 740,
                                                                                    end: 741,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "lt",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 740,
                                                                                end: 741,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 740,
                                                                    end: 741,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 730,
                                                                                end: 739,
                                                                                as_str(): "counter_4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 730,
                                                                        end: 739,
                                                                        as_str(): "counter_4",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 742,
                                                                        end: 743,
                                                                        as_str(): "7",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 730,
                                                        end: 743,
                                                        as_str(): "counter_4 < 7",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: WhileLoop(
                                                                        WhileLoopExpression {
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 770,
                                                                                                                end: 771,
                                                                                                                as_str(): "<",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 770,
                                                                                                                end: 771,
                                                                                                                as_str(): "<",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "lt",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 770,
                                                                                                            end: 771,
                                                                                                            as_str(): "<",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 770,
                                                                                                end: 771,
                                                                                                as_str(): "<",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 760,
                                                                                                            end: 769,
                                                                                                            as_str(): "counter_5",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 760,
                                                                                                    end: 769,
                                                                                                    as_str(): "counter_5",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        11,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 772,
                                                                                                    end: 774,
                                                                                                    as_str(): "11",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 760,
                                                                                    end: 774,
                                                                                    as_str(): "counter_5 < 11",
                                                                                },
                                                                            },
                                                                            body: CodeBlock {
                                                                                contents: [
                                                                                    AstNode {
                                                                                        content: Expression(
                                                                                            Expression {
                                                                                                kind: Reassignment(
                                                                                                    ReassignmentExpression {
                                                                                                        lhs: VariableExpression(
                                                                                                            Expression {
                                                                                                                kind: Variable(
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 789,
                                                                                                                            end: 798,
                                                                                                                            as_str(): "counter_5",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 789,
                                                                                                                    end: 798,
                                                                                                                    as_str(): "counter_5",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
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
                                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 811,
                                                                                                                                            end: 812,
                                                                                                                                            as_str(): "+",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 811,
                                                                                                                                            end: 812,
                                                                                                                                            as_str(): "+",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "add",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 811,
                                                                                                                                        end: 812,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 811,
                                                                                                                            end: 812,
                                                                                                                            as_str(): "+",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 801,
                                                                                                                                        end: 810,
                                                                                                                                        as_str(): "counter_5",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 801,
                                                                                                                                end: 810,
                                                                                                                                as_str(): "counter_5",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    1,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 813,
                                                                                                                                end: 814,
                                                                                                                                as_str(): "1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 801,
                                                                                                                end: 814,
                                                                                                                as_str(): "counter_5 + 1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 789,
                                                                                                    end: 814,
                                                                                                    as_str(): "counter_5 = counter_5 + 1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 789,
                                                                                            end: 814,
                                                                                            as_str(): "counter_5 = counter_5 + 1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 775,
                                                                                    end: 825,
                                                                                    as_str(): "{\n            counter_5 = counter_5 + 1;\n        }",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 754,
                                                                        end: 825,
                                                                        as_str(): "while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 754,
                                                                end: 825,
                                                                as_str(): "while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: Reassignment(
                                                                        ReassignmentExpression {
                                                                            lhs: VariableExpression(
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 834,
                                                                                                end: 843,
                                                                                                as_str(): "counter_4",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 834,
                                                                                        end: 843,
                                                                                        as_str(): "counter_4",
                                                                                    },
                                                                                },
                                                                            ),
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
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 856,
                                                                                                                end: 857,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 856,
                                                                                                                end: 857,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 856,
                                                                                                            end: 857,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 856,
                                                                                                end: 857,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 846,
                                                                                                            end: 855,
                                                                                                            as_str(): "counter_4",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 846,
                                                                                                    end: 855,
                                                                                                    as_str(): "counter_4",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 858,
                                                                                                    end: 859,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 846,
                                                                                    end: 859,
                                                                                    as_str(): "counter_4 + 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 834,
                                                                        end: 859,
                                                                        as_str(): "counter_4 = counter_4 + 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 834,
                                                                end: 859,
                                                                as_str(): "counter_4 = counter_4 + 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 744,
                                                        end: 866,
                                                        as_str(): "{\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 724,
                                            end: 866,
                                            as_str(): "while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 724,
                                    end: 866,
                                    as_str(): "while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 871,
                                                                end: 877,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 871,
                                                        end: 877,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 888,
                                                                                        end: 890,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 888,
                                                                                        end: 890,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 888,
                                                                                    end: 890,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 888,
                                                                        end: 890,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 878,
                                                                                    end: 887,
                                                                                    as_str(): "counter_5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 878,
                                                                            end: 887,
                                                                            as_str(): "counter_5",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                11,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 891,
                                                                            end: 893,
                                                                            as_str(): "11",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 878,
                                                            end: 893,
                                                            as_str(): "counter_5 == 11",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 871,
                                            end: 894,
                                            as_str(): "assert(counter_5 == 11)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 871,
                                    end: 894,
                                    as_str(): "assert(counter_5 == 11)",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 900,
                                                                end: 906,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a0e6287a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                        ),
                                                        start: 900,
                                                        end: 906,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 917,
                                                                                        end: 919,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 917,
                                                                                        end: 919,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 917,
                                                                                    end: 919,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 917,
                                                                        end: 919,
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 907,
                                                                                    end: 916,
                                                                                    as_str(): "counter_4",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 907,
                                                                            end: 916,
                                                                            as_str(): "counter_4",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 920,
                                                                            end: 921,
                                                                            as_str(): "7",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 907,
                                                            end: 921,
                                                            as_str(): "counter_4 == 7",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 900,
                                            end: 922,
                                            as_str(): "assert(counter_4 == 7)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 900,
                                    end: 922,
                                    as_str(): "assert(counter_4 == 7)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 967,
                                                    end: 973,
                                                    as_str(): "result",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: WhileLoop(
                                                    WhileLoopExpression {
                                                        condition: Expression {
                                                            kind: Literal(
                                                                Boolean(
                                                                    true,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 982,
                                                                end: 986,
                                                                as_str(): "true",
                                                            },
                                                        },
                                                        body: CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Break,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 989,
                                                                                end: 994,
                                                                                as_str(): "break",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 989,
                                                                        end: 994,
                                                                        as_str(): "break",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 987,
                                                                end: 997,
                                                                as_str(): "{ break; }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 976,
                                                    end: 997,
                                                    as_str(): "while true { break; }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 963,
                                    end: 998,
                                    as_str(): "let result = while true { break; };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 1004,
                                            end: 1008,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a0e6287a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                    ),
                                    start: 1004,
                                    end: 1008,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 53,
                            end: 1010,
                            as_str(): "{\n    let mut counter = 0;\n    // test standard while loop:\n    while counter < 10 {\n        counter = counter + 1;\n    }\n    assert(counter == 10);\n\n    // test early exit from loop with manual \"break\" (by invalidating the condition):\n    let mut counter_2 = 0;\n    let mut counter_3 = 0;\n    while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }\n\n    assert(counter_2 == 10 && counter_3 == 3);\n\n    // test nested loops:\n    let mut counter_4 = 0;\n    let mut counter_5 = 0;\n\n    while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }\n    assert(counter_5 == 11);\n    assert(counter_4 == 7);\n\n    // test while loop expression\n    let result = while true { break; };\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a0e6287a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                        ),
                        start: 35,
                        end: 1010,
                        as_str(): "fn main() -> bool {\n    let mut counter = 0;\n    // test standard while loop:\n    while counter < 10 {\n        counter = counter + 1;\n    }\n    assert(counter == 10);\n\n    // test early exit from loop with manual \"break\" (by invalidating the condition):\n    let mut counter_2 = 0;\n    let mut counter_3 = 0;\n    while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }\n\n    assert(counter_2 == 10 && counter_3 == 3);\n\n    // test nested loops:\n    let mut counter_4 = 0;\n    let mut counter_5 = 0;\n\n    while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }\n    assert(counter_5 == 11);\n    assert(counter_4 == 7);\n\n    // test while loop expression\n    let result = while true { break; };\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a0e6287a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                        ),
                        start: 48,
                        end: 52,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a0e6287a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
            ),
            start: 35,
            end: 1010,
            as_str(): "fn main() -> bool {\n    let mut counter = 0;\n    // test standard while loop:\n    while counter < 10 {\n        counter = counter + 1;\n    }\n    assert(counter == 10);\n\n    // test early exit from loop with manual \"break\" (by invalidating the condition):\n    let mut counter_2 = 0;\n    let mut counter_3 = 0;\n    while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }\n\n    assert(counter_2 == 10 && counter_3 == 3);\n\n    // test nested loops:\n    let mut counter_4 = 0;\n    let mut counter_5 = 0;\n\n    while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }\n    assert(counter_5 == 11);\n    assert(counter_4 == 7);\n\n    // test while loop expression\n    let result = while true { break; };\n\n    true\n}",
        },
    },
]
