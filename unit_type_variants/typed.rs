TyEnumDeclaration {
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
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
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
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                2,
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
            attributes: {},
        },
        TyEnumVariant {
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
            type_id: TypeId(
                5,
            ),
            initial_type_id: TypeId(
                4,
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
            attributes: {},
        },
        TyEnumVariant {
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
            type_id: TypeId(
                7,
            ),
            initial_type_id: TypeId(
                6,
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
            attributes: {},
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
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: EnumInstantiation {
                            enum_decl: TyEnumDeclaration {
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
                                type_parameters: [],
                                attributes: {},
                                variants: [
                                    TyEnumVariant {
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
                                        type_id: TypeId(
                                            3,
                                        ),
                                        initial_type_id: TypeId(
                                            2,
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
                                        attributes: {},
                                    },
                                    TyEnumVariant {
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
                                        type_id: TypeId(
                                            5,
                                        ),
                                        initial_type_id: TypeId(
                                            4,
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
                                        attributes: {},
                                    },
                                    TyEnumVariant {
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
                                        type_id: TypeId(
                                            7,
                                        ),
                                        initial_type_id: TypeId(
                                            6,
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
                                        attributes: {},
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
                            variant_name: BaseIdent {
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
                            tag: 2,
                            contents: None,
                            enum_instantiation_span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 197,
                                end: 198,
                                as_str(): "E",
                            },
                            variant_instantiation_span: Span {
                                src (ptr): 0x00007f8a28ab8be0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                ),
                                start: 200,
                                end: 201,
                                as_str(): "C",
                            },
                            type_binding: TypeBinding {
                                inner: (),
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
                        },
                        return_type: TypeId(
                            10,
                        ),
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a28ab8be0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
        ),
        start: 54,
        end: 203,
        as_str(): "fn main() -> E {\n    // Expected output is only 8 bytes because all the variants are unit types \n    //\n    //  0000000000000002  # E.tag\n\n    E::C\n}",
    },
    attributes: {},
    return_type: TypeId(
        10,
    ),
    initial_return_type: TypeId(
        9,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

