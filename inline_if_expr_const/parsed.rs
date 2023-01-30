[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
            src (ptr): 0x00007fe0ceb55870,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 38,
                            end: 39,
                            as_str(): "f",
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
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 68,
                                                                end: 72,
                                                                as_str(): "cond",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 68,
                                                        end: 72,
                                                        as_str(): "cond",
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
                                                                                    10,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ceb55870,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                ),
                                                                                start: 83,
                                                                                end: 85,
                                                                                as_str(): "10",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 83,
                                                                        end: 85,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 91,
                                                                as_str(): "{\n        10\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 91,
                                                        as_str(): "{\n        10\n    }",
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
                                                                                        20,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 109,
                                                                                    as_str(): "20",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 107,
                                                                            end: 109,
                                                                            as_str(): "20",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                    ),
                                                                    start: 97,
                                                                    end: 115,
                                                                    as_str(): "{\n        20\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 97,
                                                            end: 115,
                                                            as_str(): "{\n        20\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 115,
                                            as_str(): "if cond {\n        10\n    } else {\n        20\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 115,
                                    as_str(): "if cond {\n        10\n    } else {\n        20\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 59,
                            end: 117,
                            as_str(): "{\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 44,
                                    as_str(): "cond",
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
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007fe0ceb55870,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                ),
                                start: 46,
                                end: 50,
                                as_str(): "bool",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0ceb55870,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                        ),
                        start: 35,
                        end: 117,
                        as_str(): "fn f(cond: bool) -> u64 {\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0ceb55870,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                        ),
                        start: 55,
                        end: 58,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ceb55870,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
            ),
            start: 35,
            end: 117,
            as_str(): "fn f(cond: bool) -> u64 {\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
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
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 122,
                            end: 126,
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
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 136,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 135,
                                                        end: 136,
                                                        as_str(): "f",
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
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 137,
                                                            end: 141,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 142,
                                            as_str(): "f(true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 142,
                                    as_str(): "f(true)",
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
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 148,
                                                                end: 154,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ceb55870,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                        ),
                                                        start: 148,
                                                        end: 154,
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
                                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                        ),
                                                                                        start: 164,
                                                                                        end: 166,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                        ),
                                                                                        start: 164,
                                                                                        end: 166,
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
                                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                    ),
                                                                                    start: 164,
                                                                                    end: 166,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 164,
                                                                        end: 166,
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
                                                                                                src (ptr): 0x00007fe0ceb55870,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                                ),
                                                                                                start: 155,
                                                                                                end: 156,
                                                                                                as_str(): "f",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                        ),
                                                                                        start: 155,
                                                                                        end: 156,
                                                                                        as_str(): "f",
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
                                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                            ),
                                                                                            start: 157,
                                                                                            end: 162,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 155,
                                                                            end: 163,
                                                                            as_str(): "f(false)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                20,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 169,
                                                                            as_str(): "20",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 169,
                                                            as_str(): "f(false) == 20",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 148,
                                            end: 170,
                                            as_str(): "assert(f(false) == 20)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ceb55870,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                    ),
                                    start: 148,
                                    end: 170,
                                    as_str(): "assert(f(false) == 20)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 129,
                            end: 173,
                            as_str(): "{\n    f(true);\n    assert(f(false) == 20);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0ceb55870,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                        ),
                        start: 119,
                        end: 173,
                        as_str(): "fn main() {\n    f(true);\n    assert(f(false) == 20);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0ceb55870,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                        ),
                        start: 119,
                        end: 128,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ceb55870,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
            ),
            start: 119,
            end: 173,
            as_str(): "fn main() {\n    f(true);\n    assert(f(false) == 20);\n}",
        },
    },
]
