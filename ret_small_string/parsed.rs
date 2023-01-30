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
                            src (ptr): 0x00007fe06d0c7010,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
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
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            String(
                                                Span {
                                                    src (ptr): 0x00007fe06d0c7010,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                                    ),
                                                    start: 36,
                                                    end: 39,
                                                    as_str(): "foo",
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06d0c7010,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 40,
                                            as_str(): "\"foo\"",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06d0c7010,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                    ),
                                    start: 35,
                                    end: 40,
                                    as_str(): "\"foo\"",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06d0c7010,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                            ),
                            start: 29,
                            end: 42,
                            as_str(): "{\n    \"foo\"\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06d0c7010,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                        ),
                        start: 9,
                        end: 42,
                        as_str(): "fn main() -> str[3] {\n    \"foo\"\n}",
                    },
                    return_type: Str(
                        Length {
                            val: 3,
                            span: Span {
                                src (ptr): 0x00007fe06d0c7010,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                ),
                                start: 26,
                                end: 27,
                                as_str(): "3",
                            },
                        },
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06d0c7010,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                        ),
                        start: 22,
                        end: 28,
                        as_str(): "str[3]",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06d0c7010,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
            ),
            start: 9,
            end: 42,
            as_str(): "fn main() -> str[3] {\n    \"foo\"\n}",
        },
    },
]
