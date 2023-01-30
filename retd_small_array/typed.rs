TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cbae200,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
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
                                    src (ptr): 0x00007fe06cbae200,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 42,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    1,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe06cbae200,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                                ),
                                                start: 56,
                                                end: 57,
                                                as_str(): "1",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7259,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cbae200,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 58,
                                    as_str(): "[1]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7259,
                            ),
                            type_ascription: TypeId(
                                7255,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe06cbae200,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                    ),
                                    start: 44,
                                    end: 52,
                                    as_str(): "[u32; 1]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cbae200,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                    ),
                    start: 37,
                    end: 59,
                    as_str(): "let x: [u32; 1] = [1];",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cbae200,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 42,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe06cbae200,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                                ),
                                start: 64,
                                end: 65,
                                as_str(): "x",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7260,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06cbae200,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                            ),
                            start: 64,
                            end: 65,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06cbae200,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
                    ),
                    start: 64,
                    end: 65,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06cbae200,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
        ),
        start: 9,
        end: 67,
        as_str(): "fn main() -> [u32; 1] {\n    let x: [u32; 1] = [1];\n    x\n}",
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
        src (ptr): 0x00007fe06cbae200,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR0Ko1xU/retd_small_array/src/main.sw",
        ),
        start: 22,
        end: 30,
        as_str(): "[u32; 1]",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

