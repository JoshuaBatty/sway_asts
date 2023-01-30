[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a28ab8be0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                            ),
                            start: 14,
                            end: 15,
                            as_str(): "E",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28ab8be0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                    ),
                                    start: 22,
                                    end: 23,
                                    as_str(): "A",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 25,
                                end: 27,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 22,
                                end: 27,
                                as_str(): "A: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28ab8be0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                    ),
                                    start: 33,
                                    end: 34,
                                    as_str(): "B",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 36,
                                end: 38,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 33,
                                end: 38,
                                as_str(): "B: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a28ab8be0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                    ),
                                    start: 44,
                                    end: 45,
                                    as_str(): "C",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 47,
                                end: 49,
                                as_str(): "()",
                            },
                            tag: 2,
                            span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 44,
                                end: 49,
                                as_str(): "C: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007f8a28ab8be0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                        ),
                        start: 9,
                        end: 52,
                        as_str(): "enum E {\n    A: (),\n    B: (),\n    C: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28ab8be0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
            ),
            start: 9,
            end: 52,
            as_str(): "enum E {\n    A: (),\n    B: (),\n    C: (),\n}",
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
                            src (ptr): 0x00007f8a28ab8be0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                            ),
                            start: 57,
                            end: 61,
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
                                        kind: DelineatedPath(
                                            DelineatedPathExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28ab8be0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                    ),
                                                                    start: 197,
                                                                    end: 198,
                                                                    as_str(): "E",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 200,
                                                                end: 201,
                                                                as_str(): "C",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28ab8be0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                        ),
                                                        start: 200,
                                                        end: 201,
                                                        as_str(): "C",
                                                    },
                                                },
                                                args: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a28ab8be0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 201,
                                            as_str(): "E::C",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a28ab8be0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                    ),
                                    start: 197,
                                    end: 201,
                                    as_str(): "E::C",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a28ab8be0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                            ),
                            start: 69,
                            end: 203,
                            as_str(): "{\n    // Expected output is only 8 bytes because all the variants are unit types \n    //\n    //  0000000000000002  # E.tag\n\n    E::C\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a28ab8be0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                        ),
                        start: 54,
                        end: 203,
                        as_str(): "fn main() -> E {\n    // Expected output is only 8 bytes because all the variants are unit types \n    //\n    //  0000000000000002  # E.tag\n\n    E::C\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 67,
                                end: 68,
                                as_str(): "E",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a28ab8be0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                        ),
                        start: 67,
                        end: 68,
                        as_str(): "E",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a28ab8be0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
            ),
            start: 54,
            end: 203,
            as_str(): "fn main() -> E {\n    // Expected output is only 8 bytes because all the variants are unit types \n    //\n    //  0000000000000002  # E.tag\n\n    E::C\n}",
        },
    },
]
