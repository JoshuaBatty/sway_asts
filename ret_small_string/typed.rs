TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
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
                        return_type: TypeId(
                            4,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06d0c7010,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
        ),
        start: 9,
        end: 42,
        as_str(): "fn main() -> str[3] {\n    \"foo\"\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

