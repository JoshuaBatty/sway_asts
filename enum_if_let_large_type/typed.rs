TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 175,
            end: 181,
            as_str(): "Result",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7257),
        E: TypeId(7258),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 194,
                    end: 196,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7257,
            ),
            initial_type_id: TypeId(
                7259,
            ),
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 198,
                end: 199,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 194,
                end: 199,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 205,
                    end: 208,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7258,
            ),
            initial_type_id: TypeId(
                7260,
            ),
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 210,
                end: 211,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 205,
                end: 211,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 170,
        end: 214,
        as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
    },
    visibility: Private,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 345,
            end: 356,
            as_str(): "ItemDetails",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 363,
                    end: 367,
                    as_str(): "name",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7261,
            ),
            initial_type_id: TypeId(
                7261,
            ),
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 363,
                end: 375,
                as_str(): "name: str[4]",
            },
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 369,
                end: 375,
                as_str(): "str[4]",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 381,
                    end: 386,
                    as_str(): "price",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 381,
                end: 391,
                as_str(): "price: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 388,
                end: 391,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 338,
        end: 394,
        as_str(): "struct ItemDetails {\n    name: str[4],\n    price: u64,\n}",
    },
    attributes: {},
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 223,
            end: 230,
            as_str(): "Product",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 237,
                    end: 244,
                    as_str(): "details",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7263,
            ),
            initial_type_id: TypeId(
                7262,
            ),
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 237,
                end: 257,
                as_str(): "details: ItemDetails",
            },
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 246,
                end: 257,
                as_str(): "ItemDetails",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 263,
                    end: 279,
                    as_str(): "inventory_number",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 263,
                end: 284,
                as_str(): "inventory_number: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 281,
                end: 284,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 290,
                    end: 301,
                    as_str(): "number_sold",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 290,
                end: 306,
                as_str(): "number_sold: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 303,
                end: 306,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 312,
                    end: 328,
                    as_str(): "number_available",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 312,
                end: 333,
                as_str(): "number_available: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 330,
                end: 333,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 216,
        end: 336,
        as_str(): "struct Product {\n    details: ItemDetails,\n    inventory_number: u64,\n    number_sold: u64,\n    number_available: u64,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 401,
            end: 410,
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
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 417,
                    end: 435,
                    as_str(): "NotEnoughInventory",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7264,
            ),
            initial_type_id: TypeId(
                7264,
            ),
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 437,
                end: 443,
                as_str(): "str[3]",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 417,
                end: 443,
                as_str(): "NotEnoughInventory: str[3]",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 396,
        end: 446,
        as_str(): "enum SaleError {\n    NotEnoughInventory: str[3],\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 774,
            end: 786,
            as_str(): "sell_product",
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
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 849,
                                    end: 856,
                                    as_str(): "product",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 787,
                                            end: 794,
                                            as_str(): "product",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 859,
                                        end: 866,
                                        as_str(): "product",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7267,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 859,
                                    end: 866,
                                    as_str(): "product",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                7267,
                            ),
                            type_ascription: TypeId(
                                7271,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 841,
                    end: 867,
                    as_str(): "let mut product = product;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 900,
                                                    end: 901,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 900,
                                                    end: 901,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "lt",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 900,
                                                end: 901,
                                                as_str(): "<",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10d5eaab0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 4000,
                                                    end: 4004,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: StructFieldAccess {
                                                    prefix: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 849,
                                                                    end: 856,
                                                                    as_str(): "product",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 875,
                                                                end: 882,
                                                                as_str(): "product",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            7267,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 875,
                                                            end: 882,
                                                            as_str(): "product",
                                                        },
                                                    },
                                                    field_to_access: TyStructField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 312,
                                                                end: 328,
                                                                as_str(): "number_available",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        type_id: TypeId(
                                                            21,
                                                        ),
                                                        initial_type_id: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 333,
                                                            as_str(): "number_available: u64",
                                                        },
                                                        type_span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 330,
                                                            end: 333,
                                                            as_str(): "u64",
                                                        },
                                                        attributes: {},
                                                    },
                                                    field_instantiation_span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 883,
                                                        end: 899,
                                                        as_str(): "number_available",
                                                    },
                                                    resolved_type_of_parent: TypeId(
                                                        7267,
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 875,
                                                    end: 899,
                                                    as_str(): "product.number_available",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10d5eaab0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 4006,
                                                    end: 4011,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
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
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 902,
                                                    end: 903,
                                                    as_str(): "1",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        548,
                                        Span {
                                            src (ptr): 0x00007fb10d5eaab0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3994,
                                            end: 4129,
                                            as_str(): "fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 900,
                                                end: 901,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 875,
                                    end: 903,
                                    as_str(): "product.number_available < 1",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: EnumInstantiation {
                                                                    enum_decl: TyEnumDeclaration {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 175,
                                                                                end: 181,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_parameters: [
                                                                            T: TypeId(7267),
                                                                            E: TypeId(7269),
                                                                        ],
                                                                        attributes: {},
                                                                        variants: [
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 196,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    7267,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7259,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 198,
                                                                                    end: 199,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 194,
                                                                                    end: 199,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {},
                                                                            },
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 205,
                                                                                        end: 208,
                                                                                        as_str(): "Err",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    7269,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7260,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 210,
                                                                                    end: 211,
                                                                                    as_str(): "E",
                                                                                },
                                                                                tag: 1,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 205,
                                                                                    end: 211,
                                                                                    as_str(): "Err: E",
                                                                                },
                                                                                attributes: {},
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 170,
                                                                            end: 214,
                                                                            as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                        },
                                                                        visibility: Private,
                                                                    },
                                                                    variant_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 205,
                                                                            end: 208,
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
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 401,
                                                                                            end: 410,
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
                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 417,
                                                                                                    end: 435,
                                                                                                    as_str(): "NotEnoughInventory",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            type_id: TypeId(
                                                                                                7264,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                7264,
                                                                                            ),
                                                                                            type_span: Span {
                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                ),
                                                                                                start: 437,
                                                                                                end: 443,
                                                                                                as_str(): "str[3]",
                                                                                            },
                                                                                            tag: 0,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                ),
                                                                                                start: 417,
                                                                                                end: 443,
                                                                                                as_str(): "NotEnoughInventory: str[3]",
                                                                                            },
                                                                                            attributes: {},
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 396,
                                                                                        end: 446,
                                                                                        as_str(): "enum SaleError {\n    NotEnoughInventory: str[3],\n}",
                                                                                    },
                                                                                    visibility: Private,
                                                                                },
                                                                                variant_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 417,
                                                                                        end: 435,
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
                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 986,
                                                                                                    end: 989,
                                                                                                    as_str(): "noo",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            7283,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 985,
                                                                                            end: 990,
                                                                                            as_str(): "\"noo\"",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                enum_instantiation_span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 955,
                                                                                    end: 964,
                                                                                    as_str(): "SaleError",
                                                                                },
                                                                                variant_instantiation_span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 966,
                                                                                    end: 984,
                                                                                    as_str(): "NotEnoughInventory",
                                                                                },
                                                                                type_binding: TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 955,
                                                                                        end: 984,
                                                                                        as_str(): "SaleError::NotEnoughInventory",
                                                                                    },
                                                                                },
                                                                            },
                                                                            return_type: TypeId(
                                                                                7269,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 966,
                                                                                end: 984,
                                                                                as_str(): "NotEnoughInventory",
                                                                            },
                                                                        },
                                                                    ),
                                                                    enum_instantiation_span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 921,
                                                                        end: 927,
                                                                        as_str(): "Result",
                                                                    },
                                                                    variant_instantiation_span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 929,
                                                                        end: 932,
                                                                        as_str(): "Err",
                                                                    },
                                                                    type_binding: TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    7267,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7249,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 935,
                                                                                    end: 942,
                                                                                    as_str(): "Product",
                                                                                },
                                                                            },
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    7269,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7250,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 944,
                                                                                    end: 953,
                                                                                    as_str(): "SaleError",
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 921,
                                                                            end: 954,
                                                                            as_str(): "Result::Err::<Product, SaleError>",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7284,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 929,
                                                                    end: 932,
                                                                    as_str(): "Err",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            7285,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 914,
                                                            end: 992,
                                                            as_str(): "return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"))",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 914,
                                                    end: 992,
                                                    as_str(): "return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"))",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 904,
                                    end: 999,
                                    as_str(): "{\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    }",
                                },
                            },
                            else: None,
                        },
                        return_type: TypeId(
                            7287,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 872,
                            end: 999,
                            as_str(): "if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 872,
                    end: 999,
                    as_str(): "if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    }",
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
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 1005,
                                        end: 1012,
                                        as_str(): "product",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7267,
                                ),
                                lhs_indices: [
                                    StructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 1013,
                                                end: 1024,
                                                as_str(): "number_sold",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: FunctionApplication {
                                        call_path: CallPath {
                                            prefixes: [
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "core",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1047,
                                                        end: 1048,
                                                        as_str(): "+",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1047,
                                                        end: 1048,
                                                        as_str(): "+",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "add",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 1047,
                                                    end: 1048,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                            is_absolute: true,
                                        },
                                        contract_call_params: {},
                                        arguments: [
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d5eaab0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 124,
                                                        end: 128,
                                                        as_str(): "self",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                TyExpression {
                                                    expression: StructFieldAccess {
                                                        prefix: TyExpression {
                                                            expression: VariableExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 849,
                                                                        end: 856,
                                                                        as_str(): "product",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1027,
                                                                    end: 1034,
                                                                    as_str(): "product",
                                                                },
                                                                mutability: Mutable,
                                                            },
                                                            return_type: TypeId(
                                                                7267,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1027,
                                                                end: 1034,
                                                                as_str(): "product",
                                                            },
                                                        },
                                                        field_to_access: TyStructField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 290,
                                                                    end: 301,
                                                                    as_str(): "number_sold",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            type_id: TypeId(
                                                                21,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 290,
                                                                end: 306,
                                                                as_str(): "number_sold: u64",
                                                            },
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 303,
                                                                end: 306,
                                                                as_str(): "u64",
                                                            },
                                                            attributes: {},
                                                        },
                                                        field_instantiation_span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1035,
                                                            end: 1046,
                                                            as_str(): "number_sold",
                                                        },
                                                        resolved_type_of_parent: TypeId(
                                                            7267,
                                                        ),
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1027,
                                                        end: 1046,
                                                        as_str(): "product.number_sold",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d5eaab0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 130,
                                                        end: 135,
                                                        as_str(): "other",
                                                    },
                                                    is_raw_ident: false,
                                                },
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1049,
                                                        end: 1050,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            551,
                                            Span {
                                                src (ptr): 0x00007fb10d5eaab0,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                ),
                                                start: 117,
                                                end: 185,
                                                as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                            },
                                        ),
                                        self_state_idx: None,
                                        selector: None,
                                        type_binding: Some(
                                            TypeBinding {
                                                inner: (),
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 1047,
                                                    end: 1048,
                                                    as_str(): "+",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 1027,
                                        end: 1050,
                                        as_str(): "product.number_sold + 1",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7295,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 1005,
                            end: 1050,
                            as_str(): "product.number_sold = product.number_sold + 1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 1005,
                    end: 1050,
                    as_str(): "product.number_sold = product.number_sold + 1",
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
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 1056,
                                        end: 1063,
                                        as_str(): "product",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7267,
                                ),
                                lhs_indices: [
                                    StructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 1064,
                                                end: 1080,
                                                as_str(): "number_available",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: FunctionApplication {
                                        call_path: CallPath {
                                            prefixes: [
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "core",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1108,
                                                        end: 1109,
                                                        as_str(): "-",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1108,
                                                        end: 1109,
                                                        as_str(): "-",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            ],
                                            suffix: BaseIdent {
                                                name_override_opt: Some(
                                                    "subtract",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 1108,
                                                    end: 1109,
                                                    as_str(): "-",
                                                },
                                                is_raw_ident: false,
                                            },
                                            is_absolute: true,
                                        },
                                        contract_call_params: {},
                                        arguments: [
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d5eaab0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 581,
                                                        end: 585,
                                                        as_str(): "self",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                TyExpression {
                                                    expression: StructFieldAccess {
                                                        prefix: TyExpression {
                                                            expression: VariableExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 849,
                                                                        end: 856,
                                                                        as_str(): "product",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1083,
                                                                    end: 1090,
                                                                    as_str(): "product",
                                                                },
                                                                mutability: Mutable,
                                                            },
                                                            return_type: TypeId(
                                                                7267,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1083,
                                                                end: 1090,
                                                                as_str(): "product",
                                                            },
                                                        },
                                                        field_to_access: TyStructField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 312,
                                                                    end: 328,
                                                                    as_str(): "number_available",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            type_id: TypeId(
                                                                21,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 312,
                                                                end: 333,
                                                                as_str(): "number_available: u64",
                                                            },
                                                            type_span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 330,
                                                                end: 333,
                                                                as_str(): "u64",
                                                            },
                                                            attributes: {},
                                                        },
                                                        field_instantiation_span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1091,
                                                            end: 1107,
                                                            as_str(): "number_available",
                                                        },
                                                        resolved_type_of_parent: TypeId(
                                                            7267,
                                                        ),
                                                    },
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1083,
                                                        end: 1107,
                                                        as_str(): "product.number_available",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d5eaab0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                        ),
                                                        start: 587,
                                                        end: 592,
                                                        as_str(): "other",
                                                    },
                                                    is_raw_ident: false,
                                                },
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 1110,
                                                        end: 1111,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            552,
                                            Span {
                                                src (ptr): 0x00007fb10d5eaab0,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                ),
                                                start: 569,
                                                end: 642,
                                                as_str(): "fn subtract(self, other: Self) -> Self {\n        __sub(self, other)\n    }",
                                            },
                                        ),
                                        self_state_idx: None,
                                        selector: None,
                                        type_binding: Some(
                                            TypeBinding {
                                                inner: (),
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 1108,
                                                    end: 1109,
                                                    as_str(): "-",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 1083,
                                        end: 1111,
                                        as_str(): "product.number_available - 1",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7303,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 1056,
                            end: 1111,
                            as_str(): "product.number_available = product.number_available - 1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 1056,
                    end: 1111,
                    as_str(): "product.number_available = product.number_available - 1",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 175,
                                                end: 181,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(7306),
                                            E: TypeId(7307),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 194,
                                                        end: 196,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7306,
                                                ),
                                                initial_type_id: TypeId(
                                                    7259,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 198,
                                                    end: 199,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 194,
                                                    end: 199,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 205,
                                                        end: 208,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7307,
                                                ),
                                                initial_type_id: TypeId(
                                                    7260,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 210,
                                                    end: 211,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 211,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 214,
                                            as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 194,
                                            end: 196,
                                            as_str(): "Ok",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 849,
                                                        end: 856,
                                                        as_str(): "product",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 1135,
                                                    end: 1142,
                                                    as_str(): "product",
                                                },
                                                mutability: Mutable,
                                            },
                                            return_type: TypeId(
                                                7267,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 1135,
                                                end: 1142,
                                                as_str(): "product",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 1124,
                                        end: 1130,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 1132,
                                        end: 1134,
                                        as_str(): "Ok",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 1124,
                                            end: 1134,
                                            as_str(): "Result::Ok",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7310,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 1132,
                                    end: 1134,
                                    as_str(): "Ok",
                                },
                            },
                        ),
                        return_type: TypeId(
                            7311,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 1117,
                            end: 1143,
                            as_str(): "return Result::Ok(product)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 1117,
                    end: 1143,
                    as_str(): "return Result::Ok(product)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 787,
                    end: 794,
                    as_str(): "product",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                7267,
            ),
            initial_type_id: TypeId(
                7266,
            ),
            type_span: Span {
                src (ptr): 0x00007fb101998d50,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                ),
                start: 796,
                end: 803,
                as_str(): "Product",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 771,
        end: 1146,
        as_str(): "fn sell_product(product: Product) -> Result<Product, SaleError> {\n    let mut product = product;\n    if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    };\n    product.number_sold = product.number_sold + 1;\n    product.number_available = product.number_available - 1;\n    return Result::Ok(product);\n}",
    },
    attributes: {},
    return_type: TypeId(
        7270,
    ),
    initial_return_type: TypeId(
        7268,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 808,
        end: 834,
        as_str(): "Result<Product, SaleError>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 451,
            end: 455,
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
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 475,
                                    end: 476,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 479,
                                                end: 491,
                                                as_str(): "sell_product",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 787,
                                                    end: 794,
                                                    as_str(): "product",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: StructExpression {
                                                    struct_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 223,
                                                            end: 230,
                                                            as_str(): "Product",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    fields: [
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 510,
                                                                    end: 517,
                                                                    as_str(): "details",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: StructExpression {
                                                                    struct_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 345,
                                                                            end: 356,
                                                                            as_str(): "ItemDetails",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    fields: [
                                                                        TyStructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 545,
                                                                                    end: 549,
                                                                                    as_str(): "name",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: TyExpression {
                                                                                expression: Literal(
                                                                                    String(
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 552,
                                                                                            end: 556,
                                                                                            as_str(): "shoe",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    7319,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 551,
                                                                                    end: 557,
                                                                                    as_str(): "\"shoe\"",
                                                                                },
                                                                            },
                                                                        },
                                                                        TyStructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 559,
                                                                                    end: 564,
                                                                                    as_str(): "price",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        100,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 566,
                                                                                    end: 569,
                                                                                    as_str(): "100",
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 519,
                                                                        end: 530,
                                                                        as_str(): "ItemDetails",
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7263,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 519,
                                                                    end: 581,
                                                                    as_str(): "ItemDetails {\n            name: \"shoe\", price: 100, \n        }",
                                                                },
                                                            },
                                                        },
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 591,
                                                                    end: 607,
                                                                    as_str(): "inventory_number",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        0,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 609,
                                                                    end: 610,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 612,
                                                                    end: 623,
                                                                    as_str(): "number_sold",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        10,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 625,
                                                                    end: 627,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        },
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 629,
                                                                    end: 645,
                                                                    as_str(): "number_available",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        5,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 647,
                                                                    end: 648,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 492,
                                                        end: 499,
                                                        as_str(): "Product",
                                                    },
                                                },
                                                return_type: TypeId(
                                                    7267,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 492,
                                                    end: 654,
                                                    as_str(): "Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    }",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        556,
                                        Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 771,
                                            end: 1146,
                                            as_str(): "fn sell_product(product: Product) -> Result<Product, SaleError> {\n    let mut product = product;\n    if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    };\n    product.number_sold = product.number_sold + 1;\n    product.number_available = product.number_available - 1;\n    return Result::Ok(product);\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 479,
                                                end: 491,
                                                as_str(): "sell_product",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7270,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 479,
                                    end: 655,
                                    as_str(): "sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    })",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7270,
                            ),
                            type_ascription: TypeId(
                                7313,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 471,
                    end: 656,
                    as_str(): "let x = sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    });",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 709,
                                                    end: 710,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 709,
                                                    end: 710,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb101998d50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                ),
                                                start: 709,
                                                end: 710,
                                                as_str(): "x",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10d5eaab0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: EnumTag {
                                                    exp: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 475,
                                                                    end: 476,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 709,
                                                                end: 710,
                                                                as_str(): "x",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7270,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 709,
                                                            end: 710,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 709,
                                                    end: 710,
                                                    as_str(): "x",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10d5eaab0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 709,
                                                    end: 710,
                                                    as_str(): "x",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        558,
                                        Span {
                                            src (ptr): 0x00007fb10d5eaab0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: None,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 709,
                                    end: 710,
                                    as_str(): "x",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 704,
                                                                    end: 705,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: UnsafeDowncast {
                                                                    exp: TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 475,
                                                                                    end: 476,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 709,
                                                                                end: 710,
                                                                                as_str(): "x",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7270,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 709,
                                                                            end: 710,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    variant: TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 194,
                                                                                end: 196,
                                                                                as_str(): "Ok",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            7329,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            7259,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 198,
                                                                            end: 199,
                                                                            as_str(): "T",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 199,
                                                                            as_str(): "Ok: T",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7329,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 693,
                                                                    end: 706,
                                                                    as_str(): "Result::Ok(y)",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7329,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7329,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 704,
                                                    end: 705,
                                                    as_str(): "y",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 735,
                                                                            end: 736,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 735,
                                                                            end: 736,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "add",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 735,
                                                                        end: 736,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: true,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d5eaab0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 124,
                                                                            end: 128,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: StructFieldAccess {
                                                                            prefix: TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 704,
                                                                                            end: 705,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 721,
                                                                                        end: 722,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7329,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 721,
                                                                                    end: 722,
                                                                                    as_str(): "y",
                                                                                },
                                                                            },
                                                                            field_to_access: TyStructField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 290,
                                                                                        end: 301,
                                                                                        as_str(): "number_sold",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 290,
                                                                                    end: 306,
                                                                                    as_str(): "number_sold: u64",
                                                                                },
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 303,
                                                                                    end: 306,
                                                                                    as_str(): "u64",
                                                                                },
                                                                                attributes: {},
                                                                            },
                                                                            field_instantiation_span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 723,
                                                                                end: 734,
                                                                                as_str(): "number_sold",
                                                                            },
                                                                            resolved_type_of_parent: TypeId(
                                                                                7329,
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 721,
                                                                            end: 734,
                                                                            as_str(): "y.number_sold",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d5eaab0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 130,
                                                                            end: 135,
                                                                            as_str(): "other",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 737,
                                                                            end: 738,
                                                                            as_str(): "4",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                557,
                                                                Span {
                                                                    src (ptr): 0x00007fb10d5eaab0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 185,
                                                                    as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 735,
                                                                        end: 736,
                                                                        as_str(): "+",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 721,
                                                            end: 738,
                                                            as_str(): "y.number_sold + 4",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 721,
                                                    end: 738,
                                                    as_str(): "y.number_sold + 4",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb101998d50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                    ),
                                    start: 711,
                                    end: 744,
                                    as_str(): "{\n        y.number_sold + 4\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 760,
                                                                end: 761,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 760,
                                                        end: 761,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 750,
                                        end: 767,
                                        as_str(): "{\n        1\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 711,
                            end: 744,
                            as_str(): "{\n        y.number_sold + 4\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb101998d50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                    ),
                    start: 686,
                    end: 767,
                    as_str(): "if let Result::Ok(y) = x {\n        y.number_sold + 4\n    } else {\n        1\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 448,
        end: 769,
        as_str(): "fn main() -> u64 {\n    let x = sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    });\n\n    // should return 15\n    if let Result::Ok(y) = x {\n        y.number_sold + 4\n    } else {\n        1\n    }\n}",
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
        src (ptr): 0x00007fb101998d50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
        ),
        start: 461,
        end: 464,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

