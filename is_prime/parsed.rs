[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
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
            src (ptr): 0x00007fe0fd209f60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
            ),
            start: 9,
            end: 21,
            as_str(): "use core::*;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 26,
                            end: 29,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 31,
                            end: 37,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 39,
                            end: 45,
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
            src (ptr): 0x00007fe0fd209f60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
            ),
            start: 22,
            end: 46,
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
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 51,
                            end: 62,
                            as_str(): "check_prime",
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
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 90,
                                                                                                end: 92,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 90,
                                                                                                end: 92,
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
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 90,
                                                                                            end: 92,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 90,
                                                                                end: 92,
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
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 89,
                                                                                            as_str(): "n",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 88,
                                                                                    end: 89,
                                                                                    as_str(): "n",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 93,
                                                                                    end: 94,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 94,
                                                                    as_str(): "n == 0",
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
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 100,
                                                                                                end: 102,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 100,
                                                                                                end: 102,
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
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 100,
                                                                                            end: 102,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 100,
                                                                                end: 102,
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
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 98,
                                                                                            end: 99,
                                                                                            as_str(): "n",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 98,
                                                                                    end: 99,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 104,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 98,
                                                                    end: 104,
                                                                    as_str(): "n == 1",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 88,
                                                        end: 104,
                                                        as_str(): "n == 0 || n == 1",
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
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 115,
                                                                                end: 120,
                                                                                as_str(): "false",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 115,
                                                                        end: 120,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 126,
                                                                as_str(): "{\n        false\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 105,
                                                        end: 126,
                                                        as_str(): "{\n        false\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: Declaration(
                                                                            VariableDeclaration(
                                                                                VariableDeclaration {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 150,
                                                                                            end: 162,
                                                                                            as_str(): "is_not_prime",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_ascription: Unknown,
                                                                                    type_ascription_span: None,
                                                                                    body: Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                false,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 165,
                                                                                            end: 170,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                    is_mutable: true,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 142,
                                                                            end: 171,
                                                                            as_str(): "let mut is_not_prime = false;",
                                                                        },
                                                                    },
                                                                    AstNode {
                                                                        content: Declaration(
                                                                            VariableDeclaration(
                                                                                VariableDeclaration {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 188,
                                                                                            end: 189,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    type_ascription: Unknown,
                                                                                    type_ascription_span: None,
                                                                                    body: Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 192,
                                                                                            end: 193,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                    is_mutable: true,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 180,
                                                                            end: 194,
                                                                            as_str(): "let mut i = 2;",
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
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 211,
                                                                                                                            end: 212,
                                                                                                                            as_str(): "<",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 211,
                                                                                                                            end: 212,
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
                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 211,
                                                                                                                        end: 212,
                                                                                                                        as_str(): "<",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: true,
                                                                                                            },
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                            ),
                                                                                                            start: 211,
                                                                                                            end: 212,
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
                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 209,
                                                                                                                        end: 210,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 209,
                                                                                                                end: 210,
                                                                                                                as_str(): "i",
                                                                                                            },
                                                                                                        },
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 213,
                                                                                                                        end: 214,
                                                                                                                        as_str(): "n",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 213,
                                                                                                                end: 214,
                                                                                                                as_str(): "n",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 209,
                                                                                                end: 214,
                                                                                                as_str(): "i < n",
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
                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 239,
                                                                                                                                                        end: 241,
                                                                                                                                                        as_str(): "==",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "ops",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 239,
                                                                                                                                                        end: 241,
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
                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 239,
                                                                                                                                                    end: 241,
                                                                                                                                                    as_str(): "==",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            is_absolute: true,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 239,
                                                                                                                                        end: 241,
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
                                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 235,
                                                                                                                                                                        end: 236,
                                                                                                                                                                        as_str(): "%",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "ops",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 235,
                                                                                                                                                                        end: 236,
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
                                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 235,
                                                                                                                                                                    end: 236,
                                                                                                                                                                    as_str(): "%",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            is_absolute: true,
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    type_arguments: [],
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 235,
                                                                                                                                                        end: 236,
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
                                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 233,
                                                                                                                                                                    end: 234,
                                                                                                                                                                    as_str(): "n",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 233,
                                                                                                                                                            end: 234,
                                                                                                                                                            as_str(): "n",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Variable(
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 237,
                                                                                                                                                                    end: 238,
                                                                                                                                                                    as_str(): "i",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 237,
                                                                                                                                                            end: 238,
                                                                                                                                                            as_str(): "i",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 233,
                                                                                                                                            end: 238,
                                                                                                                                            as_str(): "n % i",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Literal(
                                                                                                                                            Numeric(
                                                                                                                                                0,
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 242,
                                                                                                                                            end: 243,
                                                                                                                                            as_str(): "0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 233,
                                                                                                                            end: 243,
                                                                                                                            as_str(): "n % i == 0",
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
                                                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 262,
                                                                                                                                                                            end: 274,
                                                                                                                                                                            as_str(): "is_not_prime",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 262,
                                                                                                                                                                    end: 274,
                                                                                                                                                                    as_str(): "is_not_prime",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        rhs: Expression {
                                                                                                                                                            kind: Literal(
                                                                                                                                                                Boolean(
                                                                                                                                                                    true,
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 277,
                                                                                                                                                                end: 281,
                                                                                                                                                                as_str(): "true",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 262,
                                                                                                                                                    end: 281,
                                                                                                                                                    as_str(): "is_not_prime = true",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 262,
                                                                                                                                            end: 281,
                                                                                                                                            as_str(): "is_not_prime = true",
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
                                                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 299,
                                                                                                                                                                            end: 300,
                                                                                                                                                                            as_str(): "i",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 299,
                                                                                                                                                                    end: 300,
                                                                                                                                                                    as_str(): "i",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        rhs: Expression {
                                                                                                                                                            kind: Variable(
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: None,
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 303,
                                                                                                                                                                        end: 304,
                                                                                                                                                                        as_str(): "n",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 303,
                                                                                                                                                                end: 304,
                                                                                                                                                                as_str(): "n",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 299,
                                                                                                                                                    end: 304,
                                                                                                                                                    as_str(): "i = n",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 299,
                                                                                                                                            end: 304,
                                                                                                                                            as_str(): "i = n",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                                whole_block_span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 244,
                                                                                                                                    end: 328,
                                                                                                                                    as_str(): "{\n                is_not_prime = true;\n                i = n; // break\n            }",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 244,
                                                                                                                            end: 328,
                                                                                                                            as_str(): "{\n                is_not_prime = true;\n                i = n; // break\n            }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    else: None,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 230,
                                                                                                                end: 328,
                                                                                                                as_str(): "if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                        ),
                                                                                                        start: 230,
                                                                                                        end: 328,
                                                                                                        as_str(): "if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            }",
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
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 342,
                                                                                                                                        end: 343,
                                                                                                                                        as_str(): "i",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 342,
                                                                                                                                end: 343,
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
                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 348,
                                                                                                                                                        end: 349,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "ops",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 348,
                                                                                                                                                        end: 349,
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
                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 348,
                                                                                                                                                    end: 349,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            is_absolute: true,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 348,
                                                                                                                                        end: 349,
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
                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 346,
                                                                                                                                                    end: 347,
                                                                                                                                                    as_str(): "i",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 346,
                                                                                                                                            end: 347,
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
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 350,
                                                                                                                                            end: 351,
                                                                                                                                            as_str(): "1",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 346,
                                                                                                                            end: 351,
                                                                                                                            as_str(): "i + 1",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 342,
                                                                                                                end: 351,
                                                                                                                as_str(): "i = i + 1",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                        ),
                                                                                                        start: 342,
                                                                                                        end: 351,
                                                                                                        as_str(): "i = i + 1",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 216,
                                                                                                end: 362,
                                                                                                as_str(): "{\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 203,
                                                                                    end: 362,
                                                                                    as_str(): "while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 203,
                                                                            end: 362,
                                                                            as_str(): "while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }",
                                                                        },
                                                                    },
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
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
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 372,
                                                                                                                end: 373,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 372,
                                                                                                                end: 373,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "not",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                            ),
                                                                                                            start: 372,
                                                                                                            end: 373,
                                                                                                            as_str(): "!",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 372,
                                                                                                end: 373,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                            ),
                                                                                                            start: 373,
                                                                                                            end: 385,
                                                                                                            as_str(): "is_not_prime",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 373,
                                                                                                    end: 385,
                                                                                                    as_str(): "is_not_prime",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 372,
                                                                                    end: 385,
                                                                                    as_str(): "!is_not_prime",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 372,
                                                                            end: 385,
                                                                            as_str(): "!is_not_prime",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 132,
                                                                    end: 391,
                                                                    as_str(): "{\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 132,
                                                            end: 391,
                                                            as_str(): "{\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 85,
                                            end: 391,
                                            as_str(): "if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 85,
                                    end: 391,
                                    as_str(): "if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 79,
                            end: 393,
                            as_str(): "{\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 63,
                                    end: 64,
                                    as_str(): "n",
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fd209f60,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                ),
                                start: 66,
                                end: 69,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fd209f60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                        ),
                        start: 48,
                        end: 393,
                        as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fd209f60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                        ),
                        start: 74,
                        end: 78,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd209f60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
            ),
            start: 48,
            end: 393,
            as_str(): "fn check_prime(n: u64) -> bool {\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
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
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 398,
                            end: 402,
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 419,
                                                                end: 425,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 419,
                                                        end: 425,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 442,
                                                                                        end: 444,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 442,
                                                                                        end: 444,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 442,
                                                                                    end: 444,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 442,
                                                                        end: 444,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 426,
                                                                                                end: 437,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 426,
                                                                                        end: 437,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                64,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 438,
                                                                                            end: 440,
                                                                                            as_str(): "64",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 426,
                                                                            end: 441,
                                                                            as_str(): "check_prime(64)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 445,
                                                                            end: 450,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 426,
                                                            end: 450,
                                                            as_str(): "check_prime(64) == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 419,
                                            end: 451,
                                            as_str(): "assert(check_prime(64) == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 419,
                                    end: 451,
                                    as_str(): "assert(check_prime(64) == false)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 457,
                                                                end: 463,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 457,
                                                        end: 463,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 479,
                                                                                        end: 481,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 479,
                                                                                        end: 481,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 479,
                                                                                    end: 481,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 479,
                                                                        end: 481,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 464,
                                                                                                end: 475,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 464,
                                                                                        end: 475,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                8,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 476,
                                                                                            end: 477,
                                                                                            as_str(): "8",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 464,
                                                                            end: 478,
                                                                            as_str(): "check_prime(8)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 482,
                                                                            end: 487,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 487,
                                                            as_str(): "check_prime(8) == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 457,
                                            end: 488,
                                            as_str(): "assert(check_prime(8) == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 457,
                                    end: 488,
                                    as_str(): "assert(check_prime(8) == false)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 494,
                                                                end: 500,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 494,
                                                        end: 500,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 516,
                                                                                        end: 518,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 516,
                                                                                        end: 518,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 516,
                                                                                    end: 518,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 516,
                                                                        end: 518,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 501,
                                                                                                end: 512,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 501,
                                                                                        end: 512,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                7,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 513,
                                                                                            end: 514,
                                                                                            as_str(): "7",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 501,
                                                                            end: 515,
                                                                            as_str(): "check_prime(7)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 519,
                                                                            end: 523,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 501,
                                                            end: 523,
                                                            as_str(): "check_prime(7) == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 494,
                                            end: 524,
                                            as_str(): "assert(check_prime(7) == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 494,
                                    end: 524,
                                    as_str(): "assert(check_prime(7) == true)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 536,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 530,
                                                        end: 536,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 553,
                                                                                        end: 555,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 553,
                                                                                        end: 555,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 553,
                                                                                    end: 555,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 553,
                                                                        end: 555,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 537,
                                                                                                end: 548,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 537,
                                                                                        end: 548,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                11,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 549,
                                                                                            end: 551,
                                                                                            as_str(): "11",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 537,
                                                                            end: 552,
                                                                            as_str(): "check_prime(11)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 556,
                                                                            end: 560,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 537,
                                                            end: 560,
                                                            as_str(): "check_prime(11) == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 530,
                                            end: 561,
                                            as_str(): "assert(check_prime(11) == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 561,
                                    as_str(): "assert(check_prime(11) == true)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 567,
                                                                end: 573,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 567,
                                                        end: 573,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 590,
                                                                                        end: 592,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 590,
                                                                                        end: 592,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 590,
                                                                                    end: 592,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 590,
                                                                        end: 592,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 574,
                                                                                                end: 585,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 574,
                                                                                        end: 585,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                13,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 586,
                                                                                            end: 588,
                                                                                            as_str(): "13",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 574,
                                                                            end: 589,
                                                                            as_str(): "check_prime(13)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 593,
                                                                            end: 597,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 597,
                                                            as_str(): "check_prime(13) == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 567,
                                            end: 598,
                                            as_str(): "assert(check_prime(13) == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 567,
                                    end: 598,
                                    as_str(): "assert(check_prime(13) == true)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 604,
                                                                end: 610,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 604,
                                                        end: 610,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 626,
                                                                                        end: 628,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 626,
                                                                                        end: 628,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 626,
                                                                                    end: 628,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 626,
                                                                        end: 628,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 611,
                                                                                                end: 622,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 611,
                                                                                        end: 622,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 623,
                                                                                            end: 624,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 611,
                                                                            end: 625,
                                                                            as_str(): "check_prime(2)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 629,
                                                                            end: 633,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 633,
                                                            as_str(): "check_prime(2) == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 604,
                                            end: 634,
                                            as_str(): "assert(check_prime(2) == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 604,
                                    end: 634,
                                    as_str(): "assert(check_prime(2) == true)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 640,
                                                                end: 646,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 640,
                                                        end: 646,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 662,
                                                                                        end: 664,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 662,
                                                                                        end: 664,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 662,
                                                                                    end: 664,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 662,
                                                                        end: 664,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 647,
                                                                                                end: 658,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 647,
                                                                                        end: 658,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                3,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 659,
                                                                                            end: 660,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 647,
                                                                            end: 661,
                                                                            as_str(): "check_prime(3)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 665,
                                                                            end: 669,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 647,
                                                            end: 669,
                                                            as_str(): "check_prime(3) == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 640,
                                            end: 670,
                                            as_str(): "assert(check_prime(3) == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 640,
                                    end: 670,
                                    as_str(): "assert(check_prime(3) == true)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 676,
                                                                end: 682,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 676,
                                                        end: 682,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 698,
                                                                                        end: 700,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 698,
                                                                                        end: 700,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 698,
                                                                                    end: 700,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 698,
                                                                        end: 700,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 683,
                                                                                                end: 694,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 683,
                                                                                        end: 694,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 695,
                                                                                            end: 696,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 683,
                                                                            end: 697,
                                                                            as_str(): "check_prime(1)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 701,
                                                                            end: 706,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 683,
                                                            end: 706,
                                                            as_str(): "check_prime(1) == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 676,
                                            end: 707,
                                            as_str(): "assert(check_prime(1) == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 676,
                                    end: 707,
                                    as_str(): "assert(check_prime(1) == false)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 713,
                                                                end: 719,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 713,
                                                        end: 719,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 735,
                                                                                        end: 737,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 735,
                                                                                        end: 737,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 735,
                                                                                    end: 737,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 735,
                                                                        end: 737,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 720,
                                                                                                end: 731,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 720,
                                                                                        end: 731,
                                                                                        as_str(): "check_prime",
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
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 732,
                                                                                            end: 733,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 720,
                                                                            end: 734,
                                                                            as_str(): "check_prime(0)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 738,
                                                                            end: 743,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 720,
                                                            end: 743,
                                                            as_str(): "check_prime(0) == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 713,
                                            end: 744,
                                            as_str(): "assert(check_prime(0) == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 713,
                                    end: 744,
                                    as_str(): "assert(check_prime(0) == false)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 751,
                                                                end: 757,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 751,
                                                        end: 757,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 774,
                                                                                        end: 776,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 774,
                                                                                        end: 776,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 774,
                                                                                    end: 776,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 774,
                                                                        end: 776,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 758,
                                                                                                end: 769,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 758,
                                                                                        end: 769,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                11,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 770,
                                                                                            end: 772,
                                                                                            as_str(): "11",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 758,
                                                                            end: 773,
                                                                            as_str(): "check_prime(11)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 777,
                                                                                                end: 788,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 777,
                                                                                        end: 788,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                17,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 789,
                                                                                            end: 791,
                                                                                            as_str(): "17",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 777,
                                                                            end: 792,
                                                                            as_str(): "check_prime(17)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 792,
                                                            as_str(): "check_prime(11) == check_prime(17)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 751,
                                            end: 793,
                                            as_str(): "assert(check_prime(11) == check_prime(17))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 751,
                                    end: 793,
                                    as_str(): "assert(check_prime(11) == check_prime(17))",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 799,
                                                                end: 805,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 799,
                                                        end: 805,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 822,
                                                                                        end: 824,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 822,
                                                                                        end: 824,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 822,
                                                                                    end: 824,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 822,
                                                                        end: 824,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 806,
                                                                                                end: 817,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 806,
                                                                                        end: 817,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                12,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 818,
                                                                                            end: 820,
                                                                                            as_str(): "12",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 806,
                                                                            end: 821,
                                                                            as_str(): "check_prime(12)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 825,
                                                                            end: 830,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 806,
                                                            end: 830,
                                                            as_str(): "check_prime(12) == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 799,
                                            end: 831,
                                            as_str(): "assert(check_prime(12) == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 799,
                                    end: 831,
                                    as_str(): "assert(check_prime(12) == false)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 837,
                                                                end: 843,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 837,
                                                        end: 843,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 860,
                                                                                        end: 862,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 860,
                                                                                        end: 862,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 860,
                                                                                    end: 862,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 860,
                                                                        end: 862,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 844,
                                                                                                end: 855,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 844,
                                                                                        end: 855,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                18,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 856,
                                                                                            end: 858,
                                                                                            as_str(): "18",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 844,
                                                                            end: 859,
                                                                            as_str(): "check_prime(18)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 863,
                                                                            end: 868,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 844,
                                                            end: 868,
                                                            as_str(): "check_prime(18) == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 837,
                                            end: 869,
                                            as_str(): "assert(check_prime(18) == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 837,
                                    end: 869,
                                    as_str(): "assert(check_prime(18) == false)",
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 875,
                                                                end: 881,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd209f60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                        ),
                                                        start: 875,
                                                        end: 881,
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
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 898,
                                                                                        end: 900,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 898,
                                                                                        end: 900,
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
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 898,
                                                                                    end: 900,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 898,
                                                                        end: 900,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 882,
                                                                                                end: 893,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 882,
                                                                                        end: 893,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                12,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 894,
                                                                                            end: 896,
                                                                                            as_str(): "12",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 882,
                                                                            end: 897,
                                                                            as_str(): "check_prime(12)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 901,
                                                                                                end: 912,
                                                                                                as_str(): "check_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 901,
                                                                                        end: 912,
                                                                                        as_str(): "check_prime",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                18,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 913,
                                                                                            end: 915,
                                                                                            as_str(): "18",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 901,
                                                                            end: 916,
                                                                            as_str(): "check_prime(18)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 882,
                                                            end: 916,
                                                            as_str(): "check_prime(12) == check_prime(18)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 875,
                                            end: 917,
                                            as_str(): "assert(check_prime(12) == check_prime(18))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 875,
                                    end: 917,
                                    as_str(): "assert(check_prime(12) == check_prime(18))",
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 924,
                                            end: 928,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd209f60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                    ),
                                    start: 924,
                                    end: 928,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 413,
                            end: 930,
                            as_str(): "{\n    assert(check_prime(64) == false);\n    assert(check_prime(8) == false);\n    assert(check_prime(7) == true);\n    assert(check_prime(11) == true);\n    assert(check_prime(13) == true);\n    assert(check_prime(2) == true);\n    assert(check_prime(3) == true);\n    assert(check_prime(1) == false);\n    assert(check_prime(0) == false);\n\n    assert(check_prime(11) == check_prime(17));\n    assert(check_prime(12) == false);\n    assert(check_prime(18) == false);\n    assert(check_prime(12) == check_prime(18));\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fd209f60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                        ),
                        start: 395,
                        end: 930,
                        as_str(): "fn main() -> bool {\n    assert(check_prime(64) == false);\n    assert(check_prime(8) == false);\n    assert(check_prime(7) == true);\n    assert(check_prime(11) == true);\n    assert(check_prime(13) == true);\n    assert(check_prime(2) == true);\n    assert(check_prime(3) == true);\n    assert(check_prime(1) == false);\n    assert(check_prime(0) == false);\n\n    assert(check_prime(11) == check_prime(17));\n    assert(check_prime(12) == false);\n    assert(check_prime(18) == false);\n    assert(check_prime(12) == check_prime(18));\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fd209f60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                        ),
                        start: 408,
                        end: 412,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd209f60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
            ),
            start: 395,
            end: 930,
            as_str(): "fn main() -> bool {\n    assert(check_prime(64) == false);\n    assert(check_prime(8) == false);\n    assert(check_prime(7) == true);\n    assert(check_prime(11) == true);\n    assert(check_prime(13) == true);\n    assert(check_prime(2) == true);\n    assert(check_prime(3) == true);\n    assert(check_prime(1) == false);\n    assert(check_prime(0) == false);\n\n    assert(check_prime(11) == check_prime(17));\n    assert(check_prime(12) == false);\n    assert(check_prime(18) == false);\n    assert(check_prime(12) == check_prime(18));\n\n    true\n}",
        },
    },
]
