[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
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
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7249),
                        E: TypeId(7250),
                    ],
                    variants: [
                        EnumVariant {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 198,
                                        end: 199,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                        },
                        EnumVariant {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 210,
                                        end: 211,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 170,
            end: 214,
            as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
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
                    attributes: {},
                    fields: [
                        StructField {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 246,
                                        end: 257,
                                        as_str(): "ItemDetails",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                        },
                        StructField {
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
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
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
                        },
                        StructField {
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
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
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
                        },
                        StructField {
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
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 216,
            end: 336,
            as_str(): "struct Product {\n    details: ItemDetails,\n    inventory_number: u64,\n    number_sold: u64,\n    number_available: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
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
                    attributes: {},
                    fields: [
                        StructField {
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
                            attributes: {},
                            type_info: Str(
                                Length {
                                    val: 4,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 373,
                                        end: 374,
                                        as_str(): "4",
                                    },
                                },
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
                        },
                        StructField {
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
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 338,
            end: 394,
            as_str(): "struct ItemDetails {\n    name: str[4],\n    price: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
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
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
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
                            attributes: {},
                            type_info: Str(
                                Length {
                                    val: 3,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 441,
                                        end: 442,
                                        as_str(): "3",
                                    },
                                },
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
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 396,
            end: 446,
            as_str(): "enum SaleError {\n    NotEnoughInventory: str[3],\n}",
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
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
                                                        arguments: [
                                                            Expression {
                                                                kind: Struct(
                                                                    StructExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 492,
                                                                                        end: 499,
                                                                                        as_str(): "Product",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
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
                                                                        fields: [
                                                                            StructExpressionField {
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
                                                                                value: Expression {
                                                                                    kind: Struct(
                                                                                        StructExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 519,
                                                                                                            end: 530,
                                                                                                            as_str(): "ItemDetails",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
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
                                                                                            fields: [
                                                                                                StructExpressionField {
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
                                                                                                    value: Expression {
                                                                                                        kind: Literal(
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
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 545,
                                                                                                        end: 557,
                                                                                                        as_str(): "name: \"shoe\"",
                                                                                                    },
                                                                                                },
                                                                                                StructExpressionField {
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
                                                                                                    value: Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                100,
                                                                                                            ),
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
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 559,
                                                                                                        end: 569,
                                                                                                        as_str(): "price: 100",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 510,
                                                                                    end: 581,
                                                                                    as_str(): "details: ItemDetails {\n            name: \"shoe\", price: 100, \n        }",
                                                                                },
                                                                            },
                                                                            StructExpressionField {
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
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 591,
                                                                                    end: 610,
                                                                                    as_str(): "inventory_number: 0",
                                                                                },
                                                                            },
                                                                            StructExpressionField {
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
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            10,
                                                                                        ),
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 612,
                                                                                    end: 627,
                                                                                    as_str(): "number_sold: 10",
                                                                                },
                                                                            },
                                                                            StructExpressionField {
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
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            5,
                                                                                        ),
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 629,
                                                                                    end: 648,
                                                                                    as_str(): "number_available: 5",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
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
                                                        ],
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Match(
                                            MatchExpression {
                                                value: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
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
                                                branches: [
                                                    MatchBranch {
                                                        scrutinee: EnumScrutinee {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 699,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 701,
                                                                        end: 703,
                                                                        as_str(): "Ok",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            value: Variable {
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
                                                                    start: 704,
                                                                    end: 705,
                                                                    as_str(): "y",
                                                                },
                                                            },
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
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: MethodApplication(
                                                                                        MethodApplicationExpression {
                                                                                            method_name_binding: TypeBinding {
                                                                                                inner: FromTrait {
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
                                                                                                },
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
                                                                                            contract_call_params: [],
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Subfield(
                                                                                                        SubfieldExpression {
                                                                                                            prefix: Expression {
                                                                                                                kind: Variable(
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 721,
                                                                                                                            end: 722,
                                                                                                                            as_str(): "y",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
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
                                                                                                            field_to_access: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 723,
                                                                                                                    end: 734,
                                                                                                                    as_str(): "number_sold",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
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
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            4,
                                                                                                        ),
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
                                                                                            ],
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
                                                                    whole_block_span: Span {
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
                                                                start: 711,
                                                                end: 744,
                                                                as_str(): "{\n        y.number_sold + 4\n    }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 744,
                                                            as_str(): "Result::Ok(y) = x {\n        y.number_sold + 4\n    }",
                                                        },
                                                    },
                                                    MatchBranch {
                                                        scrutinee: CatchAll {
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
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
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
                                                                    whole_block_span: Span {
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
                                                ],
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 465,
                            end: 769,
                            as_str(): "{\n    let x = sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    });\n\n    // should return 15\n    if let Result::Ok(y) = x {\n        y.number_sold + 4\n    } else {\n        1\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb101998d50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                        ),
                        start: 448,
                        end: 769,
                        as_str(): "fn main() -> u64 {\n    let x = sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    });\n\n    // should return 15\n    if let Result::Ok(y) = x {\n        y.number_sold + 4\n    } else {\n        1\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 448,
            end: 769,
            as_str(): "fn main() -> u64 {\n    let x = sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    });\n\n    // should return 15\n    if let Result::Ok(y) = x {\n        y.number_sold + 4\n    } else {\n        1\n    }\n}",
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 859,
                                                            end: 866,
                                                            as_str(): "product",
                                                        },
                                                        is_raw_ident: false,
                                                    },
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
                                            is_mutable: true,
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromTrait {
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
                                                                },
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
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Subfield(
                                                                        SubfieldExpression {
                                                                            prefix: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 875,
                                                                                            end: 882,
                                                                                            as_str(): "product",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
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
                                                                            field_to_access: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 883,
                                                                                    end: 899,
                                                                                    as_str(): "number_available",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
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
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
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
                                                            ],
                                                        },
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
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Return(
                                                                                Expression {
                                                                                    kind: AmbiguousPathExpression(
                                                                                        AmbiguousPathExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: AmbiguousSuffix {
                                                                                                        before: TypeBinding {
                                                                                                            inner: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 921,
                                                                                                                    end: 927,
                                                                                                                    as_str(): "Result",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 921,
                                                                                                                end: 927,
                                                                                                                as_str(): "Result",
                                                                                                            },
                                                                                                        },
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 929,
                                                                                                                end: 932,
                                                                                                                as_str(): "Err",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [
                                                                                                    TypeArgument {
                                                                                                        type_id: TypeId(
                                                                                                            7251,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            7251,
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
                                                                                                            7252,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            7252,
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
                                                                                            args: [
                                                                                                Expression {
                                                                                                    kind: AmbiguousPathExpression(
                                                                                                        AmbiguousPathExpression {
                                                                                                            call_path_binding: TypeBinding {
                                                                                                                inner: CallPath {
                                                                                                                    prefixes: [],
                                                                                                                    suffix: AmbiguousSuffix {
                                                                                                                        before: TypeBinding {
                                                                                                                            inner: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 955,
                                                                                                                                    end: 964,
                                                                                                                                    as_str(): "SaleError",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            type_arguments: [],
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 955,
                                                                                                                                end: 964,
                                                                                                                                as_str(): "SaleError",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        suffix: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 966,
                                                                                                                                end: 984,
                                                                                                                                as_str(): "NotEnoughInventory",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    is_absolute: false,
                                                                                                                },
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
                                                                                                            args: [
                                                                                                                Expression {
                                                                                                                    kind: Literal(
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
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 955,
                                                                                                        end: 991,
                                                                                                        as_str(): "SaleError::NotEnoughInventory(\"noo\")",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 921,
                                                                                        end: 992,
                                                                                        as_str(): "Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"))",
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
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 904,
                                                                end: 999,
                                                                as_str(): "{\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    }",
                                                            },
                                                        },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: Subfield(
                                                            SubfieldExpression {
                                                                prefix: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
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
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 1005,
                                                                        end: 1012,
                                                                        as_str(): "product",
                                                                    },
                                                                },
                                                                field_to_access: BaseIdent {
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
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1005,
                                                            end: 1024,
                                                            as_str(): "product.number_sold",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromTrait {
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
                                                                },
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
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Subfield(
                                                                        SubfieldExpression {
                                                                            prefix: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1027,
                                                                                            end: 1034,
                                                                                            as_str(): "product",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
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
                                                                            field_to_access: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 1035,
                                                                                    end: 1046,
                                                                                    as_str(): "number_sold",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
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
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
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
                                                            ],
                                                        },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: Subfield(
                                                            SubfieldExpression {
                                                                prefix: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
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
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 1056,
                                                                        end: 1063,
                                                                        as_str(): "product",
                                                                    },
                                                                },
                                                                field_to_access: BaseIdent {
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
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1056,
                                                            end: 1080,
                                                            as_str(): "product.number_available",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromTrait {
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
                                                                },
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
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Subfield(
                                                                        SubfieldExpression {
                                                                            prefix: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1083,
                                                                                            end: 1090,
                                                                                            as_str(): "product",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
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
                                                                            field_to_access: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 1091,
                                                                                    end: 1107,
                                                                                    as_str(): "number_available",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
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
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
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
                                                            ],
                                                        },
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 1124,
                                                                                end: 1130,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 1124,
                                                                            end: 1130,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 1132,
                                                                            end: 1134,
                                                                            as_str(): "Ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
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
                                                        args: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 1135,
                                                                            end: 1142,
                                                                            as_str(): "product",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
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
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 1124,
                                                    end: 1143,
                                                    as_str(): "Result::Ok(product)",
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 835,
                            end: 1146,
                            as_str(): "{\n    let mut product = product;\n    if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    };\n    product.number_sold = product.number_sold + 1;\n    product.number_available = product.number_available - 1;\n    return Result::Ok(product);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 796,
                                        end: 803,
                                        as_str(): "Product",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                    span: Span {
                        src (ptr): 0x00007fb101998d50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                        ),
                        start: 771,
                        end: 1146,
                        as_str(): "fn sell_product(product: Product) -> Result<Product, SaleError> {\n    let mut product = product;\n    if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    };\n    product.number_sold = product.number_sold + 1;\n    product.number_available = product.number_available - 1;\n    return Result::Ok(product);\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb101998d50,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                ),
                                start: 808,
                                end: 814,
                                as_str(): "Result",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7253,
                                    ),
                                    initial_type_id: TypeId(
                                        7253,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 815,
                                        end: 822,
                                        as_str(): "Product",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        7254,
                                    ),
                                    initial_type_id: TypeId(
                                        7254,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 824,
                                        end: 833,
                                        as_str(): "SaleError",
                                    },
                                },
                            ],
                        ),
                    },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb101998d50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
            ),
            start: 771,
            end: 1146,
            as_str(): "fn sell_product(product: Product) -> Result<Product, SaleError> {\n    let mut product = product;\n    if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    };\n    product.number_sold = product.number_sold + 1;\n    product.number_available = product.number_available - 1;\n    return Result::Ok(product);\n}",
        },
    },
]
