TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0931cfa70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
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
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0931cfa70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 42,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [],
                                },
                                return_type: TypeId(
                                    7258,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0931cfa70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 57,
                                    as_str(): "[]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7258,
                            ),
                            type_ascription: TypeId(
                                7255,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0931cfa70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                    ),
                                    start: 44,
                                    end: 52,
                                    as_str(): "[u32; 0]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0931cfa70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                    ),
                    start: 37,
                    end: 58,
                    as_str(): "let x: [u32; 0] = [];",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0931cfa70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 42,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0931cfa70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                ),
                                start: 63,
                                end: 64,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7259,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0931cfa70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                            ),
                            start: 63,
                            end: 64,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0931cfa70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                    ),
                    start: 63,
                    end: 64,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0931cfa70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
        ),
        start: 9,
        end: 66,
        as_str(): "fn main() -> [u32; 0] {\n    let x: [u32; 0] = [];\n    x\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0931cfa70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
        ),
        start: 22,
        end: 30,
        as_str(): "[u32; 0]",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

