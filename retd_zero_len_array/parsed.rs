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
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        33,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        33,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0931cfa70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                                        ),
                                                        start: 45,
                                                        end: 48,
                                                        as_str(): "u32",
                                                    },
                                                },
                                                Length {
                                                    val: 0,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0931cfa70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                                        ),
                                                        start: 50,
                                                        end: 51,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [],
                                                        length_span: None,
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0931cfa70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                                    ),
                                                    start: 63,
                                                    end: 64,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0931cfa70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                            ),
                            start: 31,
                            end: 66,
                            as_str(): "{\n    let x: [u32; 0] = [];\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0931cfa70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                        ),
                        start: 9,
                        end: 66,
                        as_str(): "fn main() -> [u32; 0] {\n    let x: [u32; 0] = [];\n    x\n}",
                    },
                    return_type: Array(
                        TypeArgument {
                            type_id: TypeId(
                                33,
                            ),
                            initial_type_id: TypeId(
                                33,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0931cfa70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                ),
                                start: 23,
                                end: 26,
                                as_str(): "u32",
                            },
                        },
                        Length {
                            val: 0,
                            span: Span {
                                src (ptr): 0x00007fe0931cfa70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
                                ),
                                start: 28,
                                end: 29,
                                as_str(): "0",
                            },
                        },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0931cfa70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR0jL3Wo/retd_zero_len_array/src/main.sw",
            ),
            start: 9,
            end: 66,
            as_str(): "fn main() -> [u32; 0] {\n    let x: [u32; 0] = [];\n    x\n}",
        },
    },
]
