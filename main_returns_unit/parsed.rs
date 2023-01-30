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
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 12,
                            end: 15,
                            as_str(): "ten",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
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
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 31,
                                            end: 33,
                                            as_str(): "10",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8c39d20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                    ),
                                    start: 31,
                                    end: 33,
                                    as_str(): "10",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 25,
                            end: 35,
                            as_str(): "{\n    10\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                        ),
                        start: 9,
                        end: 35,
                        as_str(): "fn ten() -> u64 {\n    10\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                        ),
                        start: 21,
                        end: 24,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c8c39d20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
            ),
            start: 9,
            end: 35,
            as_str(): "fn ten() -> u64 {\n    10\n}",
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
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 40,
                            end: 43,
                            as_str(): "nop",
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
                                                                src (ptr): 0x00007fe0c8c39d20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                                ),
                                                                start: 52,
                                                                end: 55,
                                                                as_str(): "ten",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c8c39d20,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                        ),
                                                        start: 52,
                                                        end: 55,
                                                        as_str(): "ten",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 52,
                                            end: 57,
                                            as_str(): "ten()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8c39d20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                    ),
                                    start: 52,
                                    end: 57,
                                    as_str(): "ten()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 46,
                            end: 60,
                            as_str(): "{\n    ten();\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                        ),
                        start: 37,
                        end: 60,
                        as_str(): "fn nop() {\n    ten();\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                        ),
                        start: 37,
                        end: 45,
                        as_str(): "fn nop()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c8c39d20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
            ),
            start: 37,
            end: 60,
            as_str(): "fn nop() {\n    ten();\n}",
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
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 65,
                            end: 69,
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
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8c39d20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                                ),
                                                                start: 78,
                                                                end: 81,
                                                                as_str(): "nop",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c8c39d20,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                        ),
                                                        start: 78,
                                                        end: 81,
                                                        as_str(): "nop",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 83,
                                            as_str(): "nop()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8c39d20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                    ),
                                    start: 78,
                                    end: 83,
                                    as_str(): "nop()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 72,
                            end: 85,
                            as_str(): "{\n    nop()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                        ),
                        start: 62,
                        end: 85,
                        as_str(): "fn main() {\n    nop()\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                        ),
                        start: 62,
                        end: 71,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c8c39d20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
            ),
            start: 62,
            end: 85,
            as_str(): "fn main() {\n    nop()\n}",
        },
    },
]
