TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            Boolean(
                                true,
                            ),
                        ),
                        return_type: TypeId(
                            71,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb135933860,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
        ),
        start: 12,
        end: 42,
        as_str(): "fn main() -> bool {\n    true\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

