[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 19,
                            end: 25,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 27,
                            end: 33,
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
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 9,
            end: 49,
            as_str(): "use std::{assert::assert, logging::log};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 35,
                            end: 42,
                            as_str(): "logging",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 44,
                            end: 47,
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
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 9,
            end: 49,
            as_str(): "use std::{assert::assert, logging::log};",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 57,
                            end: 58,
                            as_str(): "N",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Literal(
                            Numeric(
                                10,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 61,
                            end: 63,
                            as_str(): "10",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 51,
                        end: 64,
                        as_str(): "const N = 10;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 51,
            end: 64,
            as_str(): "const N = 10;",
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 69,
                            end: 86,
                            as_str(): "simple_break_test",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 103,
                                                    end: 104,
                                                    as_str(): "i",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 107,
                                                    end: 108,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 109,
                                    as_str(): "let mut i = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 120,
                                                        end: 124,
                                                        as_str(): "true",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: Expression(
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 140,
                                                                                                                end: 142,
                                                                                                                as_str(): ">=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 140,
                                                                                                                end: 142,
                                                                                                                as_str(): ">=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ge",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 140,
                                                                                                            end: 142,
                                                                                                            as_str(): ">=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 140,
                                                                                                end: 142,
                                                                                                as_str(): ">=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 138,
                                                                                                            end: 139,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 138,
                                                                                                    end: 139,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 143,
                                                                                                            end: 144,
                                                                                                            as_str(): "N",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 143,
                                                                                                    end: 144,
                                                                                                    as_str(): "N",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 138,
                                                                                    end: 144,
                                                                                    as_str(): "i >= N",
                                                                                },
                                                                            },
                                                                            then: Expression {
                                                                                kind: CodeBlock(
                                                                                    CodeBlock {
                                                                                        contents: [
                                                                                            AstNode {
                                                                                                content: Expression(
                                                                                                    Expression {
                                                                                                        kind: Break,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 159,
                                                                                                            end: 164,
                                                                                                            as_str(): "break",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 159,
                                                                                                    end: 164,
                                                                                                    as_str(): "break",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 145,
                                                                                            end: 175,
                                                                                            as_str(): "{\n            break;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 175,
                                                                                    as_str(): "{\n            break;\n        }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 135,
                                                                        end: 175,
                                                                        as_str(): "if i >= N {\n            break;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 175,
                                                                as_str(): "if i >= N {\n            break;\n        }",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: ImplicitReturnExpression(
                                                                Expression {
                                                                    kind: Reassignment(
                                                                        ReassignmentExpression {
                                                                            lhs: VariableExpression(
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 184,
                                                                                                end: 185,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 184,
                                                                                        end: 185,
                                                                                        as_str(): "i",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 186,
                                                                                                                end: 188,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 186,
                                                                                                                end: 188,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 186,
                                                                                                            end: 188,
                                                                                                            as_str(): "+=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 186,
                                                                                                end: 188,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 184,
                                                                                                            end: 185,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 184,
                                                                                                    end: 185,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 189,
                                                                                                    end: 190,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 190,
                                                                                    as_str(): "i += 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 184,
                                                                        end: 190,
                                                                        as_str(): "i += 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 184,
                                                                end: 190,
                                                                as_str(): "i += 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 125,
                                                        end: 196,
                                                        as_str(): "{\n        if i >= N {\n            break;\n        }\n        i += 1\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 114,
                                            end: 196,
                                            as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 196,
                                    as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 201,
                                                                end: 207,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 201,
                                                        end: 207,
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 210,
                                                                                        end: 212,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 210,
                                                                                        end: 212,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 210,
                                                                                    end: 212,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 210,
                                                                        end: 212,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 208,
                                                                                    end: 209,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 208,
                                                                            end: 209,
                                                                            as_str(): "i",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): "N",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 213,
                                                                            end: 214,
                                                                            as_str(): "N",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 214,
                                                            as_str(): "i == N",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 201,
                                            end: 215,
                                            as_str(): "assert(i == N)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 201,
                                    end: 215,
                                    as_str(): "assert(i == N)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 89,
                            end: 218,
                            as_str(): "{\n    let mut i = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }\n    assert(i == N);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 66,
                        end: 218,
                        as_str(): "fn simple_break_test() {\n    let mut i = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }\n    assert(i == N);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 66,
                        end: 88,
                        as_str(): "fn simple_break_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 66,
            end: 218,
            as_str(): "fn simple_break_test() {\n    let mut i = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }\n    assert(i == N);\n}",
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 223,
                            end: 243,
                            as_str(): "simple_continue_test",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 261,
                                                    as_str(): "i",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 264,
                                                    end: 265,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 252,
                                    end: 266,
                                    as_str(): "let mut i = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 279,
                                                    end: 282,
                                                    as_str(): "sum",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 285,
                                                    end: 286,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 271,
                                    end: 287,
                                    as_str(): "let mut sum = 0;",
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 300,
                                                                                    end: 301,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 300,
                                                                                    end: 301,
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 300,
                                                                                end: 301,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 300,
                                                                    end: 301,
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
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 298,
                                                                                end: 299,
                                                                                as_str(): "i",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 298,
                                                                        end: 299,
                                                                        as_str(): "i",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 302,
                                                                                end: 303,
                                                                                as_str(): "N",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 302,
                                                                        end: 303,
                                                                        as_str(): "N",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 303,
                                                        as_str(): "i < N",
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 314,
                                                                                                end: 315,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 314,
                                                                                        end: 315,
                                                                                        as_str(): "i",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 316,
                                                                                                                end: 318,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 316,
                                                                                                                end: 318,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 316,
                                                                                                            end: 318,
                                                                                                            as_str(): "+=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 316,
                                                                                                end: 318,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 314,
                                                                                                            end: 315,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 314,
                                                                                                    end: 315,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 319,
                                                                                                    end: 320,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 314,
                                                                                    end: 320,
                                                                                    as_str(): "i += 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 314,
                                                                        end: 320,
                                                                        as_str(): "i += 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 314,
                                                                end: 320,
                                                                as_str(): "i += 1",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: Expression(
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 339,
                                                                                                                end: 341,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 339,
                                                                                                                end: 341,
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
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 339,
                                                                                                            end: 341,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 339,
                                                                                                end: 341,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
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
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 335,
                                                                                                                                end: 336,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "ops",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 335,
                                                                                                                                end: 336,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    suffix: BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "modulo",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 335,
                                                                                                                            end: 336,
                                                                                                                            as_str(): "%",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    is_absolute: true,
                                                                                                                },
                                                                                                            },
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 335,
                                                                                                                end: 336,
                                                                                                                as_str(): "%",
                                                                                                            },
                                                                                                        },
                                                                                                        contract_call_params: [],
                                                                                                        arguments: [
                                                                                                            Expression {
                                                                                                                kind: Variable(
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 333,
                                                                                                                            end: 334,
                                                                                                                            as_str(): "i",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 333,
                                                                                                                    end: 334,
                                                                                                                    as_str(): "i",
                                                                                                                },
                                                                                                            },
                                                                                                            Expression {
                                                                                                                kind: Literal(
                                                                                                                    Numeric(
                                                                                                                        2,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 337,
                                                                                                                    end: 338,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                            },
                                                                                                        ],
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 333,
                                                                                                    end: 338,
                                                                                                    as_str(): "i % 2",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 342,
                                                                                                    end: 343,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 333,
                                                                                    end: 343,
                                                                                    as_str(): "i % 2 == 0",
                                                                                },
                                                                            },
                                                                            then: Expression {
                                                                                kind: CodeBlock(
                                                                                    CodeBlock {
                                                                                        contents: [
                                                                                            AstNode {
                                                                                                content: Expression(
                                                                                                    Expression {
                                                                                                        kind: Continue,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 358,
                                                                                                            end: 366,
                                                                                                            as_str(): "continue",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 358,
                                                                                                    end: 366,
                                                                                                    as_str(): "continue",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 344,
                                                                                            end: 377,
                                                                                            as_str(): "{\n            continue;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 344,
                                                                                    end: 377,
                                                                                    as_str(): "{\n            continue;\n        }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 330,
                                                                        end: 377,
                                                                        as_str(): "if i % 2 == 0 {\n            continue;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 330,
                                                                end: 377,
                                                                as_str(): "if i % 2 == 0 {\n            continue;\n        }",
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 386,
                                                                                                end: 389,
                                                                                                as_str(): "sum",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 386,
                                                                                        end: 389,
                                                                                        as_str(): "sum",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 390,
                                                                                                                end: 392,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 390,
                                                                                                                end: 392,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 390,
                                                                                                            end: 392,
                                                                                                            as_str(): "+=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 390,
                                                                                                end: 392,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 386,
                                                                                                            end: 389,
                                                                                                            as_str(): "sum",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 386,
                                                                                                    end: 389,
                                                                                                    as_str(): "sum",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 393,
                                                                                                    end: 394,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 386,
                                                                                    end: 394,
                                                                                    as_str(): "sum += 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 386,
                                                                        end: 394,
                                                                        as_str(): "sum += 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 386,
                                                                end: 394,
                                                                as_str(): "sum += 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 304,
                                                        end: 401,
                                                        as_str(): "{\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 292,
                                            end: 401,
                                            as_str(): "while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 292,
                                    end: 401,
                                    as_str(): "while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 406,
                                                                end: 412,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 406,
                                                        end: 412,
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 417,
                                                                                        end: 419,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 417,
                                                                                        end: 419,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 417,
                                                                                    end: 419,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 417,
                                                                        end: 419,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 413,
                                                                                    end: 416,
                                                                                    as_str(): "sum",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 413,
                                                                            end: 416,
                                                                            as_str(): "sum",
                                                                        },
                                                                    },
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
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 422,
                                                                                                        end: 423,
                                                                                                        as_str(): "/",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 422,
                                                                                                        end: 423,
                                                                                                        as_str(): "/",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "divide",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 422,
                                                                                                    end: 423,
                                                                                                    as_str(): "/",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 422,
                                                                                        end: 423,
                                                                                        as_str(): "/",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 420,
                                                                                                    end: 421,
                                                                                                    as_str(): "N",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 420,
                                                                                            end: 421,
                                                                                            as_str(): "N",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 424,
                                                                                            end: 425,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 420,
                                                                            end: 425,
                                                                            as_str(): "N / 2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 413,
                                                            end: 425,
                                                            as_str(): "sum == N / 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 406,
                                            end: 426,
                                            as_str(): "assert(sum == N / 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 406,
                                    end: 426,
                                    as_str(): "assert(sum == N / 2)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 246,
                            end: 429,
                            as_str(): "{\n    let mut i = 0;\n    let mut sum = 0;\n    while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }\n    assert(sum == N / 2);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 220,
                        end: 429,
                        as_str(): "fn simple_continue_test() {\n    let mut i = 0;\n    let mut sum = 0;\n    while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }\n    assert(sum == N / 2);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 220,
                        end: 245,
                        as_str(): "fn simple_continue_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 220,
            end: 429,
            as_str(): "fn simple_continue_test() {\n    let mut i = 0;\n    let mut sum = 0;\n    while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }\n    assert(sum == N / 2);\n}",
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 434,
                            end: 457,
                            as_str(): "break_and_continue_test",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 474,
                                                    end: 475,
                                                    as_str(): "i",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 478,
                                                    end: 479,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 466,
                                    end: 480,
                                    as_str(): "let mut i = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 493,
                                                    end: 494,
                                                    as_str(): "j",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 497,
                                                    end: 498,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 485,
                                    end: 499,
                                    as_str(): "let mut j = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 512,
                                                    end: 513,
                                                    as_str(): "k",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 516,
                                                    end: 517,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 504,
                                    end: 518,
                                    as_str(): "let mut k = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 531,
                                                    end: 532,
                                                    as_str(): "n",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 535,
                                                    end: 536,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 523,
                                    end: 537,
                                    as_str(): "let mut n = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 550,
                                                    end: 554,
                                                    as_str(): "sum1",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 557,
                                                    end: 558,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 542,
                                    end: 559,
                                    as_str(): "let mut sum1 = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 572,
                                                    end: 576,
                                                    as_str(): "sum2",
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
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 579,
                                                    end: 580,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 564,
                                    end: 581,
                                    as_str(): "let mut sum2 = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 592,
                                                        end: 596,
                                                        as_str(): "true",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: Expression(
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 612,
                                                                                                                end: 614,
                                                                                                                as_str(): ">=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 612,
                                                                                                                end: 614,
                                                                                                                as_str(): ">=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ge",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 612,
                                                                                                            end: 614,
                                                                                                            as_str(): ">=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 612,
                                                                                                end: 614,
                                                                                                as_str(): ">=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 610,
                                                                                                            end: 611,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 610,
                                                                                                    end: 611,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 615,
                                                                                                            end: 616,
                                                                                                            as_str(): "N",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 615,
                                                                                                    end: 616,
                                                                                                    as_str(): "N",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 610,
                                                                                    end: 616,
                                                                                    as_str(): "i >= N",
                                                                                },
                                                                            },
                                                                            then: Expression {
                                                                                kind: CodeBlock(
                                                                                    CodeBlock {
                                                                                        contents: [
                                                                                            AstNode {
                                                                                                content: Expression(
                                                                                                    Expression {
                                                                                                        kind: Break,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 631,
                                                                                                            end: 636,
                                                                                                            as_str(): "break",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 631,
                                                                                                    end: 636,
                                                                                                    as_str(): "break",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 617,
                                                                                            end: 647,
                                                                                            as_str(): "{\n            break;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 617,
                                                                                    end: 647,
                                                                                    as_str(): "{\n            break;\n        }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 607,
                                                                        end: 647,
                                                                        as_str(): "if i >= N {\n            break;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 607,
                                                                end: 647,
                                                                as_str(): "if i >= N {\n            break;\n        }",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: WhileLoop(
                                                                        WhileLoopExpression {
                                                                            condition: Expression {
                                                                                kind: Literal(
                                                                                    Boolean(
                                                                                        true,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 662,
                                                                                    end: 666,
                                                                                    as_str(): "true",
                                                                                },
                                                                            },
                                                                            body: CodeBlock {
                                                                                contents: [
                                                                                    AstNode {
                                                                                        content: Expression(
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 686,
                                                                                                                                            end: 688,
                                                                                                                                            as_str(): ">=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 686,
                                                                                                                                            end: 688,
                                                                                                                                            as_str(): ">=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "ge",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 686,
                                                                                                                                        end: 688,
                                                                                                                                        as_str(): ">=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 686,
                                                                                                                            end: 688,
                                                                                                                            as_str(): ">=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 684,
                                                                                                                                        end: 685,
                                                                                                                                        as_str(): "j",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 684,
                                                                                                                                end: 685,
                                                                                                                                as_str(): "j",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 689,
                                                                                                                                        end: 690,
                                                                                                                                        as_str(): "N",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 689,
                                                                                                                                end: 690,
                                                                                                                                as_str(): "N",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 684,
                                                                                                                end: 690,
                                                                                                                as_str(): "j >= N",
                                                                                                            },
                                                                                                        },
                                                                                                        then: Expression {
                                                                                                            kind: CodeBlock(
                                                                                                                CodeBlock {
                                                                                                                    contents: [
                                                                                                                        AstNode {
                                                                                                                            content: Expression(
                                                                                                                                Expression {
                                                                                                                                    kind: Break,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 709,
                                                                                                                                        end: 714,
                                                                                                                                        as_str(): "break",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 709,
                                                                                                                                end: 714,
                                                                                                                                as_str(): "break",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    whole_block_span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 691,
                                                                                                                        end: 729,
                                                                                                                        as_str(): "{\n                break;\n            }",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 691,
                                                                                                                end: 729,
                                                                                                                as_str(): "{\n                break;\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                        else: None,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 681,
                                                                                                    end: 729,
                                                                                                    as_str(): "if j >= N {\n                break;\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 681,
                                                                                            end: 729,
                                                                                            as_str(): "if j >= N {\n                break;\n            }",
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
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 742,
                                                                                                                            end: 746,
                                                                                                                            as_str(): "sum1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 742,
                                                                                                                    end: 746,
                                                                                                                    as_str(): "sum1",
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 747,
                                                                                                                                            end: 749,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 747,
                                                                                                                                            end: 749,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "add",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 747,
                                                                                                                                        end: 749,
                                                                                                                                        as_str(): "+=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 747,
                                                                                                                            end: 749,
                                                                                                                            as_str(): "+=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 742,
                                                                                                                                        end: 746,
                                                                                                                                        as_str(): "sum1",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 742,
                                                                                                                                end: 746,
                                                                                                                                as_str(): "sum1",
                                                                                                                            },
                                                                                                                        },
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
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 756,
                                                                                                                                                            end: 757,
                                                                                                                                                            as_str(): "+",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                            "ops",
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 756,
                                                                                                                                                            end: 757,
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
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 756,
                                                                                                                                                        end: 757,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                is_absolute: true,
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        type_arguments: [],
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 756,
                                                                                                                                            end: 757,
                                                                                                                                            as_str(): "+",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    contract_call_params: [],
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
                                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 752,
                                                                                                                                                                            end: 753,
                                                                                                                                                                            as_str(): "*",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                    BaseIdent {
                                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                                            "ops",
                                                                                                                                                                        ),
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 752,
                                                                                                                                                                            end: 753,
                                                                                                                                                                            as_str(): "*",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                ],
                                                                                                                                                                suffix: BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "multiply",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 752,
                                                                                                                                                                        end: 753,
                                                                                                                                                                        as_str(): "*",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                is_absolute: true,
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                        type_arguments: [],
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 752,
                                                                                                                                                            end: 753,
                                                                                                                                                            as_str(): "*",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    contract_call_params: [],
                                                                                                                                                    arguments: [
                                                                                                                                                        Expression {
                                                                                                                                                            kind: Variable(
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: None,
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 750,
                                                                                                                                                                        end: 751,
                                                                                                                                                                        as_str(): "i",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 750,
                                                                                                                                                                end: 751,
                                                                                                                                                                as_str(): "i",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                        Expression {
                                                                                                                                                            kind: Variable(
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: None,
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 754,
                                                                                                                                                                        end: 755,
                                                                                                                                                                        as_str(): "N",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 754,
                                                                                                                                                                end: 755,
                                                                                                                                                                as_str(): "N",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    ],
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 750,
                                                                                                                                                end: 755,
                                                                                                                                                as_str(): "i * N",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        Expression {
                                                                                                                                            kind: Variable(
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 758,
                                                                                                                                                        end: 759,
                                                                                                                                                        as_str(): "j",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 758,
                                                                                                                                                end: 759,
                                                                                                                                                as_str(): "j",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 750,
                                                                                                                                end: 759,
                                                                                                                                as_str(): "i * N + j",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 742,
                                                                                                                end: 759,
                                                                                                                as_str(): "sum1 += i * N + j",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 742,
                                                                                                    end: 759,
                                                                                                    as_str(): "sum1 += i * N + j",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 742,
                                                                                            end: 759,
                                                                                            as_str(): "sum1 += i * N + j",
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
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 773,
                                                                                                                            end: 774,
                                                                                                                            as_str(): "j",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 773,
                                                                                                                    end: 774,
                                                                                                                    as_str(): "j",
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 775,
                                                                                                                                            end: 777,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 775,
                                                                                                                                            end: 777,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "add",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 775,
                                                                                                                                        end: 777,
                                                                                                                                        as_str(): "+=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 775,
                                                                                                                            end: 777,
                                                                                                                            as_str(): "+=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 773,
                                                                                                                                        end: 774,
                                                                                                                                        as_str(): "j",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 773,
                                                                                                                                end: 774,
                                                                                                                                as_str(): "j",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    1,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 778,
                                                                                                                                end: 779,
                                                                                                                                as_str(): "1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 773,
                                                                                                                end: 779,
                                                                                                                as_str(): "j += 1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 773,
                                                                                                    end: 779,
                                                                                                    as_str(): "j += 1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 773,
                                                                                            end: 779,
                                                                                            as_str(): "j += 1",
                                                                                        },
                                                                                    },
                                                                                    AstNode {
                                                                                        content: Expression(
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 803,
                                                                                                                                            end: 805,
                                                                                                                                            as_str(): "==",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 803,
                                                                                                                                            end: 805,
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
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 803,
                                                                                                                                        end: 805,
                                                                                                                                        as_str(): "==",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 803,
                                                                                                                            end: 805,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
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
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 799,
                                                                                                                                                            end: 800,
                                                                                                                                                            as_str(): "%",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                            "ops",
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 799,
                                                                                                                                                            end: 800,
                                                                                                                                                            as_str(): "%",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                                suffix: BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "modulo",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 799,
                                                                                                                                                        end: 800,
                                                                                                                                                        as_str(): "%",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                is_absolute: true,
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        type_arguments: [],
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 799,
                                                                                                                                            end: 800,
                                                                                                                                            as_str(): "%",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    contract_call_params: [],
                                                                                                                                    arguments: [
                                                                                                                                        Expression {
                                                                                                                                            kind: Variable(
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 797,
                                                                                                                                                        end: 798,
                                                                                                                                                        as_str(): "j",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 797,
                                                                                                                                                end: 798,
                                                                                                                                                as_str(): "j",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        Expression {
                                                                                                                                            kind: Literal(
                                                                                                                                                Numeric(
                                                                                                                                                    2,
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 801,
                                                                                                                                                end: 802,
                                                                                                                                                as_str(): "2",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 797,
                                                                                                                                end: 802,
                                                                                                                                as_str(): "j % 2",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    0,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 806,
                                                                                                                                end: 807,
                                                                                                                                as_str(): "0",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 797,
                                                                                                                end: 807,
                                                                                                                as_str(): "j % 2 == 0",
                                                                                                            },
                                                                                                        },
                                                                                                        then: Expression {
                                                                                                            kind: CodeBlock(
                                                                                                                CodeBlock {
                                                                                                                    contents: [
                                                                                                                        AstNode {
                                                                                                                            content: Expression(
                                                                                                                                Expression {
                                                                                                                                    kind: Continue,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 826,
                                                                                                                                        end: 834,
                                                                                                                                        as_str(): "continue",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 826,
                                                                                                                                end: 834,
                                                                                                                                as_str(): "continue",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    whole_block_span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 808,
                                                                                                                        end: 849,
                                                                                                                        as_str(): "{\n                continue;\n            }",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 808,
                                                                                                                end: 849,
                                                                                                                as_str(): "{\n                continue;\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                        else: None,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 794,
                                                                                                    end: 849,
                                                                                                    as_str(): "if j % 2 == 0 {\n                continue;\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 794,
                                                                                            end: 849,
                                                                                            as_str(): "if j % 2 == 0 {\n                continue;\n            }",
                                                                                        },
                                                                                    },
                                                                                    AstNode {
                                                                                        content: ImplicitReturnExpression(
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 871,
                                                                                                                                            end: 872,
                                                                                                                                            as_str(): "<",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 871,
                                                                                                                                            end: 872,
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
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 871,
                                                                                                                                        end: 872,
                                                                                                                                        as_str(): "<",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 871,
                                                                                                                            end: 872,
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
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 869,
                                                                                                                                        end: 870,
                                                                                                                                        as_str(): "n",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 869,
                                                                                                                                end: 870,
                                                                                                                                as_str(): "n",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 873,
                                                                                                                                        end: 874,
                                                                                                                                        as_str(): "N",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 873,
                                                                                                                                end: 874,
                                                                                                                                as_str(): "N",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 869,
                                                                                                                end: 874,
                                                                                                                as_str(): "n < N",
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
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 893,
                                                                                                                                                        end: 897,
                                                                                                                                                        as_str(): "sum1",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 893,
                                                                                                                                                end: 897,
                                                                                                                                                as_str(): "sum1",
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
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 898,
                                                                                                                                                                        end: 900,
                                                                                                                                                                        as_str(): "+=",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "ops",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 898,
                                                                                                                                                                        end: 900,
                                                                                                                                                                        as_str(): "+=",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                    "add",
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 898,
                                                                                                                                                                    end: 900,
                                                                                                                                                                    as_str(): "+=",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            is_absolute: true,
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    type_arguments: [],
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 898,
                                                                                                                                                        end: 900,
                                                                                                                                                        as_str(): "+=",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                contract_call_params: [],
                                                                                                                                                arguments: [
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Variable(
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 893,
                                                                                                                                                                    end: 897,
                                                                                                                                                                    as_str(): "sum1",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 893,
                                                                                                                                                            end: 897,
                                                                                                                                                            as_str(): "sum1",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Variable(
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 901,
                                                                                                                                                                    end: 902,
                                                                                                                                                                    as_str(): "n",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 901,
                                                                                                                                                            end: 902,
                                                                                                                                                            as_str(): "n",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 893,
                                                                                                                                            end: 902,
                                                                                                                                            as_str(): "sum1 += n",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 893,
                                                                                                                                end: 902,
                                                                                                                                as_str(): "sum1 += n",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 893,
                                                                                                                        end: 902,
                                                                                                                        as_str(): "sum1 += n",
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
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 920,
                                                                                                                                                        end: 921,
                                                                                                                                                        as_str(): "n",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 920,
                                                                                                                                                end: 921,
                                                                                                                                                as_str(): "n",
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
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 922,
                                                                                                                                                                        end: 924,
                                                                                                                                                                        as_str(): "+=",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "ops",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 922,
                                                                                                                                                                        end: 924,
                                                                                                                                                                        as_str(): "+=",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                    "add",
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 922,
                                                                                                                                                                    end: 924,
                                                                                                                                                                    as_str(): "+=",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            is_absolute: true,
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    type_arguments: [],
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 922,
                                                                                                                                                        end: 924,
                                                                                                                                                        as_str(): "+=",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                contract_call_params: [],
                                                                                                                                                arguments: [
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Variable(
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 920,
                                                                                                                                                                    end: 921,
                                                                                                                                                                    as_str(): "n",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 920,
                                                                                                                                                            end: 921,
                                                                                                                                                            as_str(): "n",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Literal(
                                                                                                                                                            Numeric(
                                                                                                                                                                1,
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 925,
                                                                                                                                                            end: 926,
                                                                                                                                                            as_str(): "1",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 920,
                                                                                                                                            end: 926,
                                                                                                                                            as_str(): "n += 1",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 920,
                                                                                                                                end: 926,
                                                                                                                                as_str(): "n += 1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 920,
                                                                                                                        end: 926,
                                                                                                                        as_str(): "n += 1",
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
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 952,
                                                                                                                                                                        end: 953,
                                                                                                                                                                        as_str(): ">",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "ops",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 952,
                                                                                                                                                                        end: 953,
                                                                                                                                                                        as_str(): ">",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                    "gt",
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 952,
                                                                                                                                                                    end: 953,
                                                                                                                                                                    as_str(): ">",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            is_absolute: true,
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    type_arguments: [],
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 952,
                                                                                                                                                        end: 953,
                                                                                                                                                        as_str(): ">",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                contract_call_params: [],
                                                                                                                                                arguments: [
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Variable(
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 947,
                                                                                                                                                                    end: 951,
                                                                                                                                                                    as_str(): "sum1",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 947,
                                                                                                                                                            end: 951,
                                                                                                                                                            as_str(): "sum1",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Literal(
                                                                                                                                                            Numeric(
                                                                                                                                                                50,
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 954,
                                                                                                                                                            end: 956,
                                                                                                                                                            as_str(): "50",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 947,
                                                                                                                                            end: 956,
                                                                                                                                            as_str(): "sum1 > 50",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    then: Expression {
                                                                                                                                        kind: CodeBlock(
                                                                                                                                            CodeBlock {
                                                                                                                                                contents: [
                                                                                                                                                    AstNode {
                                                                                                                                                        content: Expression(
                                                                                                                                                            Expression {
                                                                                                                                                                kind: Break,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 979,
                                                                                                                                                                    end: 984,
                                                                                                                                                                    as_str(): "break",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 979,
                                                                                                                                                            end: 984,
                                                                                                                                                            as_str(): "break",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                                whole_block_span: Span {
                                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 957,
                                                                                                                                                    end: 1003,
                                                                                                                                                    as_str(): "{\n                    break;\n                }",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 957,
                                                                                                                                            end: 1003,
                                                                                                                                            as_str(): "{\n                    break;\n                }",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    else: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 944,
                                                                                                                                end: 1003,
                                                                                                                                as_str(): "if sum1 > 50 {\n                    break;\n                }",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 944,
                                                                                                                        end: 1003,
                                                                                                                        as_str(): "if sum1 > 50 {\n                    break;\n                }",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                            whole_block_span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 875,
                                                                                                                end: 1017,
                                                                                                                as_str(): "{\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 863,
                                                                                                    end: 1017,
                                                                                                    as_str(): "while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 863,
                                                                                            end: 1017,
                                                                                            as_str(): "while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 667,
                                                                                    end: 1027,
                                                                                    as_str(): "{\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 656,
                                                                        end: 1027,
                                                                        as_str(): "while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 656,
                                                                end: 1027,
                                                                as_str(): "while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: WhileLoop(
                                                                        WhileLoopExpression {
                                                                            condition: Expression {
                                                                                kind: Literal(
                                                                                    Boolean(
                                                                                        true,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1043,
                                                                                    end: 1047,
                                                                                    as_str(): "true",
                                                                                },
                                                                            },
                                                                            body: CodeBlock {
                                                                                contents: [
                                                                                    AstNode {
                                                                                        content: Expression(
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1067,
                                                                                                                                            end: 1069,
                                                                                                                                            as_str(): ">=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1067,
                                                                                                                                            end: 1069,
                                                                                                                                            as_str(): ">=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "ge",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1067,
                                                                                                                                        end: 1069,
                                                                                                                                        as_str(): ">=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1067,
                                                                                                                            end: 1069,
                                                                                                                            as_str(): ">=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1065,
                                                                                                                                        end: 1066,
                                                                                                                                        as_str(): "k",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1065,
                                                                                                                                end: 1066,
                                                                                                                                as_str(): "k",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1070,
                                                                                                                                        end: 1071,
                                                                                                                                        as_str(): "N",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1070,
                                                                                                                                end: 1071,
                                                                                                                                as_str(): "N",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1065,
                                                                                                                end: 1071,
                                                                                                                as_str(): "k >= N",
                                                                                                            },
                                                                                                        },
                                                                                                        then: Expression {
                                                                                                            kind: CodeBlock(
                                                                                                                CodeBlock {
                                                                                                                    contents: [
                                                                                                                        AstNode {
                                                                                                                            content: Expression(
                                                                                                                                Expression {
                                                                                                                                    kind: Break,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1090,
                                                                                                                                        end: 1095,
                                                                                                                                        as_str(): "break",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1090,
                                                                                                                                end: 1095,
                                                                                                                                as_str(): "break",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    whole_block_span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1072,
                                                                                                                        end: 1110,
                                                                                                                        as_str(): "{\n                break;\n            }",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1072,
                                                                                                                end: 1110,
                                                                                                                as_str(): "{\n                break;\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                        else: None,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1062,
                                                                                                    end: 1110,
                                                                                                    as_str(): "if k >= N {\n                break;\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1062,
                                                                                            end: 1110,
                                                                                            as_str(): "if k >= N {\n                break;\n            }",
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
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1123,
                                                                                                                            end: 1127,
                                                                                                                            as_str(): "sum1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1123,
                                                                                                                    end: 1127,
                                                                                                                    as_str(): "sum1",
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1128,
                                                                                                                                            end: 1130,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1128,
                                                                                                                                            end: 1130,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "add",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1128,
                                                                                                                                        end: 1130,
                                                                                                                                        as_str(): "+=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1128,
                                                                                                                            end: 1130,
                                                                                                                            as_str(): "+=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1123,
                                                                                                                                        end: 1127,
                                                                                                                                        as_str(): "sum1",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1123,
                                                                                                                                end: 1127,
                                                                                                                                as_str(): "sum1",
                                                                                                                            },
                                                                                                                        },
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
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1137,
                                                                                                                                                            end: 1138,
                                                                                                                                                            as_str(): "+",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                            "ops",
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1137,
                                                                                                                                                            end: 1138,
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
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1137,
                                                                                                                                                        end: 1138,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                is_absolute: true,
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        type_arguments: [],
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1137,
                                                                                                                                            end: 1138,
                                                                                                                                            as_str(): "+",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    contract_call_params: [],
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
                                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 1133,
                                                                                                                                                                            end: 1134,
                                                                                                                                                                            as_str(): "*",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                    BaseIdent {
                                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                                            "ops",
                                                                                                                                                                        ),
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 1133,
                                                                                                                                                                            end: 1134,
                                                                                                                                                                            as_str(): "*",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                ],
                                                                                                                                                                suffix: BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "multiply",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1133,
                                                                                                                                                                        end: 1134,
                                                                                                                                                                        as_str(): "*",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                is_absolute: true,
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                        type_arguments: [],
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1133,
                                                                                                                                                            end: 1134,
                                                                                                                                                            as_str(): "*",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    contract_call_params: [],
                                                                                                                                                    arguments: [
                                                                                                                                                        Expression {
                                                                                                                                                            kind: Variable(
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: None,
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1131,
                                                                                                                                                                        end: 1132,
                                                                                                                                                                        as_str(): "i",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1131,
                                                                                                                                                                end: 1132,
                                                                                                                                                                as_str(): "i",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                        Expression {
                                                                                                                                                            kind: Variable(
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: None,
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1135,
                                                                                                                                                                        end: 1136,
                                                                                                                                                                        as_str(): "N",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1135,
                                                                                                                                                                end: 1136,
                                                                                                                                                                as_str(): "N",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    ],
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1131,
                                                                                                                                                end: 1136,
                                                                                                                                                as_str(): "i * N",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        Expression {
                                                                                                                                            kind: Variable(
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1139,
                                                                                                                                                        end: 1140,
                                                                                                                                                        as_str(): "k",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1139,
                                                                                                                                                end: 1140,
                                                                                                                                                as_str(): "k",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1131,
                                                                                                                                end: 1140,
                                                                                                                                as_str(): "i * N + k",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1123,
                                                                                                                end: 1140,
                                                                                                                as_str(): "sum1 += i * N + k",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1123,
                                                                                                    end: 1140,
                                                                                                    as_str(): "sum1 += i * N + k",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1123,
                                                                                            end: 1140,
                                                                                            as_str(): "sum1 += i * N + k",
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
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1154,
                                                                                                                            end: 1155,
                                                                                                                            as_str(): "k",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1154,
                                                                                                                    end: 1155,
                                                                                                                    as_str(): "k",
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1156,
                                                                                                                                            end: 1158,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1156,
                                                                                                                                            end: 1158,
                                                                                                                                            as_str(): "+=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "add",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1156,
                                                                                                                                        end: 1158,
                                                                                                                                        as_str(): "+=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1156,
                                                                                                                            end: 1158,
                                                                                                                            as_str(): "+=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1154,
                                                                                                                                        end: 1155,
                                                                                                                                        as_str(): "k",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1154,
                                                                                                                                end: 1155,
                                                                                                                                as_str(): "k",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    1,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1159,
                                                                                                                                end: 1160,
                                                                                                                                as_str(): "1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1154,
                                                                                                                end: 1160,
                                                                                                                as_str(): "k += 1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1154,
                                                                                                    end: 1160,
                                                                                                    as_str(): "k += 1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1154,
                                                                                            end: 1160,
                                                                                            as_str(): "k += 1",
                                                                                        },
                                                                                    },
                                                                                    AstNode {
                                                                                        content: Expression(
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1184,
                                                                                                                                            end: 1186,
                                                                                                                                            as_str(): "==",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1184,
                                                                                                                                            end: 1186,
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
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1184,
                                                                                                                                        end: 1186,
                                                                                                                                        as_str(): "==",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1184,
                                                                                                                            end: 1186,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
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
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1180,
                                                                                                                                                            end: 1181,
                                                                                                                                                            as_str(): "%",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                            "ops",
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1180,
                                                                                                                                                            end: 1181,
                                                                                                                                                            as_str(): "%",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                                suffix: BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "modulo",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1180,
                                                                                                                                                        end: 1181,
                                                                                                                                                        as_str(): "%",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                is_absolute: true,
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        type_arguments: [],
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1180,
                                                                                                                                            end: 1181,
                                                                                                                                            as_str(): "%",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    contract_call_params: [],
                                                                                                                                    arguments: [
                                                                                                                                        Expression {
                                                                                                                                            kind: Variable(
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1178,
                                                                                                                                                        end: 1179,
                                                                                                                                                        as_str(): "k",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1178,
                                                                                                                                                end: 1179,
                                                                                                                                                as_str(): "k",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        Expression {
                                                                                                                                            kind: Literal(
                                                                                                                                                Numeric(
                                                                                                                                                    2,
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1182,
                                                                                                                                                end: 1183,
                                                                                                                                                as_str(): "2",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1178,
                                                                                                                                end: 1183,
                                                                                                                                as_str(): "k % 2",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    0,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1187,
                                                                                                                                end: 1188,
                                                                                                                                as_str(): "0",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1178,
                                                                                                                end: 1188,
                                                                                                                as_str(): "k % 2 == 0",
                                                                                                            },
                                                                                                        },
                                                                                                        then: Expression {
                                                                                                            kind: CodeBlock(
                                                                                                                CodeBlock {
                                                                                                                    contents: [
                                                                                                                        AstNode {
                                                                                                                            content: Expression(
                                                                                                                                Expression {
                                                                                                                                    kind: Continue,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1207,
                                                                                                                                        end: 1215,
                                                                                                                                        as_str(): "continue",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1207,
                                                                                                                                end: 1215,
                                                                                                                                as_str(): "continue",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    whole_block_span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1189,
                                                                                                                        end: 1230,
                                                                                                                        as_str(): "{\n                continue;\n            }",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1189,
                                                                                                                end: 1230,
                                                                                                                as_str(): "{\n                continue;\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                        else: None,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1175,
                                                                                                    end: 1230,
                                                                                                    as_str(): "if k % 2 == 0 {\n                continue;\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1175,
                                                                                            end: 1230,
                                                                                            as_str(): "if k % 2 == 0 {\n                continue;\n            }",
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
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1244,
                                                                                                                            end: 1248,
                                                                                                                            as_str(): "sum1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1244,
                                                                                                                    end: 1248,
                                                                                                                    as_str(): "sum1",
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
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1249,
                                                                                                                                            end: 1251,
                                                                                                                                            as_str(): "*=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "ops",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1249,
                                                                                                                                            end: 1251,
                                                                                                                                            as_str(): "*=",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "multiply",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1249,
                                                                                                                                        end: 1251,
                                                                                                                                        as_str(): "*=",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                is_absolute: true,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1249,
                                                                                                                            end: 1251,
                                                                                                                            as_str(): "*=",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    contract_call_params: [],
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1244,
                                                                                                                                        end: 1248,
                                                                                                                                        as_str(): "sum1",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1244,
                                                                                                                                end: 1248,
                                                                                                                                as_str(): "sum1",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    2,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1252,
                                                                                                                                end: 1253,
                                                                                                                                as_str(): "2",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1244,
                                                                                                                end: 1253,
                                                                                                                as_str(): "sum1 *= 2",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1244,
                                                                                                    end: 1253,
                                                                                                    as_str(): "sum1 *= 2",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1244,
                                                                                            end: 1253,
                                                                                            as_str(): "sum1 *= 2",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                whole_block_span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1048,
                                                                                    end: 1264,
                                                                                    as_str(): "{\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1037,
                                                                        end: 1264,
                                                                        as_str(): "while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1037,
                                                                end: 1264,
                                                                as_str(): "while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }",
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1273,
                                                                                                end: 1274,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1273,
                                                                                        end: 1274,
                                                                                        as_str(): "i",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1275,
                                                                                                                end: 1277,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1275,
                                                                                                                end: 1277,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1275,
                                                                                                            end: 1277,
                                                                                                            as_str(): "+=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1275,
                                                                                                end: 1277,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1273,
                                                                                                            end: 1274,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1273,
                                                                                                    end: 1274,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1278,
                                                                                                    end: 1279,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1273,
                                                                                    end: 1279,
                                                                                    as_str(): "i += 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1273,
                                                                        end: 1279,
                                                                        as_str(): "i += 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1273,
                                                                end: 1279,
                                                                as_str(): "i += 1",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: Expression(
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1299,
                                                                                                                end: 1301,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1299,
                                                                                                                end: 1301,
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
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1299,
                                                                                                            end: 1301,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1299,
                                                                                                end: 1301,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
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
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1295,
                                                                                                                                end: 1296,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "ops",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1295,
                                                                                                                                end: 1296,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    suffix: BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "modulo",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1295,
                                                                                                                            end: 1296,
                                                                                                                            as_str(): "%",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    is_absolute: true,
                                                                                                                },
                                                                                                            },
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1295,
                                                                                                                end: 1296,
                                                                                                                as_str(): "%",
                                                                                                            },
                                                                                                        },
                                                                                                        contract_call_params: [],
                                                                                                        arguments: [
                                                                                                            Expression {
                                                                                                                kind: Variable(
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1293,
                                                                                                                            end: 1294,
                                                                                                                            as_str(): "i",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1293,
                                                                                                                    end: 1294,
                                                                                                                    as_str(): "i",
                                                                                                                },
                                                                                                            },
                                                                                                            Expression {
                                                                                                                kind: Literal(
                                                                                                                    Numeric(
                                                                                                                        2,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1297,
                                                                                                                    end: 1298,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                            },
                                                                                                        ],
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1293,
                                                                                                    end: 1298,
                                                                                                    as_str(): "i % 2",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1302,
                                                                                                    end: 1303,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1293,
                                                                                    end: 1303,
                                                                                    as_str(): "i % 2 == 0",
                                                                                },
                                                                            },
                                                                            then: Expression {
                                                                                kind: CodeBlock(
                                                                                    CodeBlock {
                                                                                        contents: [
                                                                                            AstNode {
                                                                                                content: Expression(
                                                                                                    Expression {
                                                                                                        kind: Continue,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1318,
                                                                                                            end: 1326,
                                                                                                            as_str(): "continue",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1318,
                                                                                                    end: 1326,
                                                                                                    as_str(): "continue",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1304,
                                                                                            end: 1337,
                                                                                            as_str(): "{\n            continue;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1304,
                                                                                    end: 1337,
                                                                                    as_str(): "{\n            continue;\n        }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1290,
                                                                        end: 1337,
                                                                        as_str(): "if i % 2 == 0 {\n            continue;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1290,
                                                                end: 1337,
                                                                as_str(): "if i % 2 == 0 {\n            continue;\n        }",
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1347,
                                                                                                end: 1351,
                                                                                                as_str(): "sum1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1347,
                                                                                        end: 1351,
                                                                                        as_str(): "sum1",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1352,
                                                                                                                end: 1354,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1352,
                                                                                                                end: 1354,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1352,
                                                                                                            end: 1354,
                                                                                                            as_str(): "+=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1352,
                                                                                                end: 1354,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1347,
                                                                                                            end: 1351,
                                                                                                            as_str(): "sum1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1347,
                                                                                                    end: 1351,
                                                                                                    as_str(): "sum1",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1355,
                                                                                                    end: 1356,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1347,
                                                                                    end: 1356,
                                                                                    as_str(): "sum1 += 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1347,
                                                                        end: 1356,
                                                                        as_str(): "sum1 += 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1347,
                                                                end: 1356,
                                                                as_str(): "sum1 += 1",
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
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1366,
                                                                                                end: 1370,
                                                                                                as_str(): "sum2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1366,
                                                                                        end: 1370,
                                                                                        as_str(): "sum2",
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
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1371,
                                                                                                                end: 1373,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1371,
                                                                                                                end: 1373,
                                                                                                                as_str(): "+=",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1371,
                                                                                                            end: 1373,
                                                                                                            as_str(): "+=",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1371,
                                                                                                end: 1373,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1366,
                                                                                                            end: 1370,
                                                                                                            as_str(): "sum2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1366,
                                                                                                    end: 1370,
                                                                                                    as_str(): "sum2",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1374,
                                                                                                    end: 1375,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1366,
                                                                                    end: 1375,
                                                                                    as_str(): "sum2 += 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1366,
                                                                        end: 1375,
                                                                        as_str(): "sum2 += 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1366,
                                                                end: 1375,
                                                                as_str(): "sum2 += 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 597,
                                                        end: 1382,
                                                        as_str(): "{\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 586,
                                            end: 1382,
                                            as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 586,
                                    end: 1382,
                                    as_str(): "while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1388,
                                                                end: 1394,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1388,
                                                        end: 1394,
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1400,
                                                                                        end: 1402,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1400,
                                                                                        end: 1402,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1400,
                                                                                    end: 1402,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1400,
                                                                        end: 1402,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1395,
                                                                                    end: 1399,
                                                                                    as_str(): "sum1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1395,
                                                                            end: 1399,
                                                                            as_str(): "sum1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3072,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1403,
                                                                            end: 1407,
                                                                            as_str(): "3072",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1395,
                                                            end: 1407,
                                                            as_str(): "sum1 == 3072",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1388,
                                            end: 1408,
                                            as_str(): "assert(sum1 == 3072)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 1388,
                                    end: 1408,
                                    as_str(): "assert(sum1 == 3072)",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1414,
                                                                end: 1420,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1414,
                                                        end: 1420,
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
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1426,
                                                                                        end: 1428,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1426,
                                                                                        end: 1428,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1426,
                                                                                    end: 1428,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1426,
                                                                        end: 1428,
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
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1421,
                                                                                    end: 1425,
                                                                                    as_str(): "sum2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1421,
                                                                            end: 1425,
                                                                            as_str(): "sum2",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1429,
                                                                            end: 1430,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1421,
                                                            end: 1430,
                                                            as_str(): "sum2 == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1414,
                                            end: 1431,
                                            as_str(): "assert(sum2 == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 1414,
                                    end: 1431,
                                    as_str(): "assert(sum2 == 5)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 460,
                            end: 1434,
                            as_str(): "{\n    let mut i = 0;\n    let mut j = 0;\n    let mut k = 0;\n    let mut n = 0;\n    let mut sum1 = 0;\n    let mut sum2 = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }\n\n    assert(sum1 == 3072);\n    assert(sum2 == 5);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 431,
                        end: 1434,
                        as_str(): "fn break_and_continue_test() {\n    let mut i = 0;\n    let mut j = 0;\n    let mut k = 0;\n    let mut n = 0;\n    let mut sum1 = 0;\n    let mut sum2 = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }\n\n    assert(sum1 == 3072);\n    assert(sum2 == 5);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 431,
                        end: 459,
                        as_str(): "fn break_and_continue_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 431,
            end: 1434,
            as_str(): "fn break_and_continue_test() {\n    let mut i = 0;\n    let mut j = 0;\n    let mut k = 0;\n    let mut n = 0;\n    let mut sum1 = 0;\n    let mut sum2 = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }\n\n    assert(sum1 == 3072);\n    assert(sum2 == 5);\n}",
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
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1439,
                            end: 1443,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1460,
                                                                end: 1477,
                                                                as_str(): "simple_break_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1460,
                                                        end: 1477,
                                                        as_str(): "simple_break_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1460,
                                            end: 1479,
                                            as_str(): "simple_break_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 1460,
                                    end: 1479,
                                    as_str(): "simple_break_test()",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1485,
                                                                end: 1505,
                                                                as_str(): "simple_continue_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1485,
                                                        end: 1505,
                                                        as_str(): "simple_continue_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1485,
                                            end: 1507,
                                            as_str(): "simple_continue_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 1485,
                                    end: 1507,
                                    as_str(): "simple_continue_test()",
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
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1513,
                                                                end: 1536,
                                                                as_str(): "break_and_continue_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12b9f78e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                        ),
                                                        start: 1513,
                                                        end: 1536,
                                                        as_str(): "break_and_continue_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1513,
                                            end: 1538,
                                            as_str(): "break_and_continue_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 1513,
                                    end: 1538,
                                    as_str(): "break_and_continue_test()",
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
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1545,
                                            end: 1549,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12b9f78e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                    ),
                                    start: 1545,
                                    end: 1549,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 1454,
                            end: 1551,
                            as_str(): "{\n    simple_break_test();\n    simple_continue_test();\n    break_and_continue_test();\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 1436,
                        end: 1551,
                        as_str(): "fn main() -> bool {\n    simple_break_test();\n    simple_continue_test();\n    break_and_continue_test();\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 1449,
                        end: 1453,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12b9f78e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
            ),
            start: 1436,
            end: 1551,
            as_str(): "fn main() -> bool {\n    simple_break_test();\n    simple_continue_test();\n    break_and_continue_test();\n\n    true\n}",
        },
    },
]
