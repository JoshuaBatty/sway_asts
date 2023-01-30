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
                            src (ptr): 0x00007fb135933860,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                            ),
                            start: 15,
                            end: 19,
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
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb135933860,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                            ),
                                            start: 36,
                                            end: 40,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb135933860,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 40,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb135933860,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                            ),
                            start: 30,
                            end: 42,
                            as_str(): "{\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb135933860,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                        ),
                        start: 12,
                        end: 42,
                        as_str(): "fn main() -> bool {\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb135933860,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                        ),
                        start: 25,
                        end: 29,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb135933860,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
            ),
            start: 12,
            end: 42,
            as_str(): "fn main() -> bool {\n    true\n}",
        },
    },
]
