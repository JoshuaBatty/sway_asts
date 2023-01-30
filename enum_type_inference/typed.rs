TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 175,
            end: 181,
            as_str(): "Result",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7255),
        E: TypeId(7256),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 192,
                    end: 194,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7255,
            ),
            initial_type_id: TypeId(
                7257,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0ed8e1980,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                ),
                start: 196,
                end: 197,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb0ed8e1980,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                ),
                start: 192,
                end: 197,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 201,
                    end: 204,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7256,
            ),
            initial_type_id: TypeId(
                7258,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0ed8e1980,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                ),
                start: 206,
                end: 207,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb0ed8e1980,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                ),
                start: 201,
                end: 207,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb0ed8e1980,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
        ),
        start: 170,
        end: 210,
        as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
    },
    visibility: Private,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 219,
            end: 226,
            as_str(): "Product",
        },
        is_raw_ident: false,
    },
    fields: [],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb0ed8e1980,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
        ),
        start: 212,
        end: 230,
        as_str(): "struct Product {\n}",
    },
    attributes: {},
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 239,
            end: 250,
            as_str(): "ItemDetails",
        },
        is_raw_ident: false,
    },
    fields: [],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb0ed8e1980,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
        ),
        start: 232,
        end: 254,
        as_str(): "struct ItemDetails {\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 261,
            end: 270,
            as_str(): "SaleError",
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
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 277,
                    end: 295,
                    as_str(): "NotEnoughInventory",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7259,
            ),
            initial_type_id: TypeId(
                7259,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0ed8e1980,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                ),
                start: 297,
                end: 303,
                as_str(): "str[3]",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb0ed8e1980,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                ),
                start: 277,
                end: 303,
                as_str(): "NotEnoughInventory: str[3]",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb0ed8e1980,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
        ),
        start: 256,
        end: 307,
        as_str(): "enum SaleError {\n    NotEnoughInventory: str[3], \n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ed8e1980,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
            ),
            start: 312,
            end: 316,
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
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 336,
                                    end: 337,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0ed8e1980,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                ),
                                                start: 175,
                                                end: 181,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(7262),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 192,
                                                        end: 194,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7257,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 196,
                                                    end: 197,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 197,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 201,
                                                        end: 204,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7262,
                                                ),
                                                initial_type_id: TypeId(
                                                    7258,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 206,
                                                    end: 207,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 201,
                                                    end: 207,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 210,
                                            as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 192,
                                            end: 194,
                                            as_str(): "Ok",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    5,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0ed8e1980,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                ),
                                                start: 369,
                                                end: 373,
                                                as_str(): "5u64",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 346,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 348,
                                        end: 350,
                                        as_str(): "Ok",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [
                                            TypeArgument {
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7249,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 353,
                                                    end: 356,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    7262,
                                                ),
                                                initial_type_id: TypeId(
                                                    7250,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 358,
                                                    end: 367,
                                                    as_str(): "SaleError",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 340,
                                            end: 368,
                                            as_str(): "Result::Ok::<u64, SaleError>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7265,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 348,
                                    end: 350,
                                    as_str(): "Ok",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7265,
                            ),
                            type_ascription: TypeId(
                                7261,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 332,
                    end: 375,
                    as_str(): "let x = Result::Ok::<u64, SaleError>(5u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 388,
                                    end: 389,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0ed8e1980,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                ),
                                                start: 175,
                                                end: 181,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(7262),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 192,
                                                        end: 194,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7257,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 196,
                                                    end: 197,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 197,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 201,
                                                        end: 204,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7262,
                                                ),
                                                initial_type_id: TypeId(
                                                    7258,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 206,
                                                    end: 207,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 201,
                                                    end: 207,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 210,
                                            as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 201,
                                            end: 204,
                                            as_str(): "Err",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
                                    contents: Some(
                                        TyExpression {
                                            expression: EnumInstantiation {
                                                enum_decl: TyEnumDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 261,
                                                            end: 270,
                                                            as_str(): "SaleError",
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
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 277,
                                                                    end: 295,
                                                                    as_str(): "NotEnoughInventory",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            type_id: TypeId(
                                                                7259,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                7259,
                                                            ),
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 297,
                                                                end: 303,
                                                                as_str(): "str[3]",
                                                            },
                                                            tag: 0,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 277,
                                                                end: 303,
                                                                as_str(): "NotEnoughInventory: str[3]",
                                                            },
                                                            attributes: {},
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 256,
                                                        end: 307,
                                                        as_str(): "enum SaleError {\n    NotEnoughInventory: str[3], \n}",
                                                    },
                                                    visibility: Private,
                                                },
                                                variant_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 277,
                                                        end: 295,
                                                        as_str(): "NotEnoughInventory",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                tag: 0,
                                                contents: Some(
                                                    TyExpression {
                                                        expression: Literal(
                                                            String(
                                                                Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 456,
                                                                    as_str(): "foo",
                                                                },
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            7270,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 452,
                                                            end: 457,
                                                            as_str(): "\"foo\"",
                                                        },
                                                    },
                                                ),
                                                enum_instantiation_span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 422,
                                                    end: 431,
                                                    as_str(): "SaleError",
                                                },
                                                variant_instantiation_span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 433,
                                                    end: 451,
                                                    as_str(): "NotEnoughInventory",
                                                },
                                                type_binding: TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 422,
                                                        end: 451,
                                                        as_str(): "SaleError::NotEnoughInventory",
                                                    },
                                                },
                                            },
                                            return_type: TypeId(
                                                7262,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0ed8e1980,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                ),
                                                start: 433,
                                                end: 451,
                                                as_str(): "NotEnoughInventory",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 392,
                                        end: 398,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 400,
                                        end: 403,
                                        as_str(): "Err",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [
                                            TypeArgument {
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7249,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 406,
                                                    end: 409,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    7262,
                                                ),
                                                initial_type_id: TypeId(
                                                    7250,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 411,
                                                    end: 420,
                                                    as_str(): "SaleError",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 392,
                                            end: 421,
                                            as_str(): "Result::Err::<u64, SaleError>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7271,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0ed8e1980,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                    ),
                                    start: 400,
                                    end: 403,
                                    as_str(): "Err",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                7271,
                            ),
                            type_ascription: TypeId(
                                7266,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 380,
                    end: 460,
                    as_str(): "let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 496,
                                        end: 497,
                                        as_str(): "y",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7271,
                                ),
                                lhs_indices: [],
                                rhs: TyExpression {
                                    expression: VariableExpression {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0ed8e1980,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                ),
                                                start: 336,
                                                end: 337,
                                                as_str(): "x",
                                            },
                                            is_raw_ident: false,
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 500,
                                            end: 501,
                                            as_str(): "x",
                                        },
                                        mutability: Immutable,
                                    },
                                    return_type: TypeId(
                                        7265,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 500,
                                        end: 501,
                                        as_str(): "x",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7275,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 496,
                            end: 501,
                            as_str(): "y = x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 496,
                    end: 501,
                    as_str(): "y = x",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                5,
                            ),
                        ),
                        return_type: TypeId(
                            7276,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 507,
                            end: 508,
                            as_str(): "5",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0ed8e1980,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                    ),
                    start: 507,
                    end: 508,
                    as_str(): "5",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0ed8e1980,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
        ),
        start: 309,
        end: 510,
        as_str(): "fn main() -> u64 {\n    let x = Result::Ok::<u64, SaleError>(5u64);\n    let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));\n    // should be the same type\n    y = x;\n    5\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb0ed8e1980,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
        ),
        start: 322,
        end: 325,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

