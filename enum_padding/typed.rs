TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 18,
            end: 32,
            as_str(): "LowerLevelEnum",
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
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 39,
                    end: 44,
                    as_str(): "first",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                0,
            ),
            initial_type_id: TypeId(
                0,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 46,
                end: 50,
                as_str(): "b256",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 39,
                end: 50,
                as_str(): "first: b256",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 56,
                    end: 62,
                    as_str(): "second",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 64,
                end: 67,
                as_str(): "u32",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 56,
                end: 67,
                as_str(): "second: u32",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb0ee12f040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
        ),
        start: 9,
        end: 70,
        as_str(): "pub enum LowerLevelEnum {\n    first: b256,\n    second: u32,\n}",
    },
    visibility: Public,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 83,
            end: 94,
            as_str(): "ThenAStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 101,
                    end: 106,
                    as_str(): "first",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
            span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 101,
                end: 111,
                as_str(): "first: u32",
            },
            type_span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 108,
                end: 111,
                as_str(): "u32",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 117,
                    end: 123,
                    as_str(): "second",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                5,
            ),
            initial_type_id: TypeId(
                4,
            ),
            span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 117,
                end: 139,
                as_str(): "second: LowerLevelEnum",
            },
            type_span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 125,
                end: 139,
                as_str(): "LowerLevelEnum",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Public,
    span: Span {
        src (ptr): 0x00007fb0ee12f040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
        ),
        start: 72,
        end: 142,
        as_str(): "pub struct ThenAStruct {\n    first: u32,\n    second: LowerLevelEnum,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 153,
            end: 165,
            as_str(): "TopLevelEnum",
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
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 172,
                    end: 177,
                    as_str(): "first",
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
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 179,
                end: 195,
                as_str(): "(b256,\n    b256)",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 172,
                end: 195,
                as_str(): "first: (b256,\n    b256)",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 197,
                    end: 203,
                    as_str(): "second",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                9,
            ),
            initial_type_id: TypeId(
                8,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 205,
                end: 216,
                as_str(): "ThenAStruct",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb0ee12f040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                ),
                start: 197,
                end: 216,
                as_str(): "second: ThenAStruct",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb0ee12f040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
        ),
        start: 144,
        end: 219,
        as_str(): "pub enum TopLevelEnum {\n    first: (b256,\n    b256), second: ThenAStruct,\n}",
    },
    visibility: Public,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0ee12f040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
            ),
            start: 224,
            end: 228,
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
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 153,
                                        end: 165,
                                        as_str(): "TopLevelEnum",
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
                                                src (ptr): 0x00007fb0ee12f040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                ),
                                                start: 172,
                                                end: 177,
                                                as_str(): "first",
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
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 179,
                                            end: 195,
                                            as_str(): "(b256,\n    b256)",
                                        },
                                        tag: 0,
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 172,
                                            end: 195,
                                            as_str(): "first: (b256,\n    b256)",
                                        },
                                        attributes: {},
                                    },
                                    TyEnumVariant {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0ee12f040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                ),
                                                start: 197,
                                                end: 203,
                                                as_str(): "second",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            9,
                                        ),
                                        initial_type_id: TypeId(
                                            8,
                                        ),
                                        type_span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 205,
                                            end: 216,
                                            as_str(): "ThenAStruct",
                                        },
                                        tag: 1,
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 216,
                                            as_str(): "second: ThenAStruct",
                                        },
                                        attributes: {},
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 144,
                                    end: 219,
                                    as_str(): "pub enum TopLevelEnum {\n    first: (b256,\n    b256), second: ThenAStruct,\n}",
                                },
                                visibility: Public,
                            },
                            variant_name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 197,
                                    end: 203,
                                    as_str(): "second",
                                },
                                is_raw_ident: false,
                            },
                            tag: 1,
                            contents: Some(
                                TyExpression {
                                    expression: StructExpression {
                                        struct_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0ee12f040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                ),
                                                start: 83,
                                                end: 94,
                                                as_str(): "ThenAStruct",
                                            },
                                            is_raw_ident: false,
                                        },
                                        fields: [
                                            TyStructExpressionField {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 883,
                                                        end: 888,
                                                        as_str(): "first",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                value: TyExpression {
                                                    expression: Literal(
                                                        U64(
                                                            42,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        17,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 890,
                                                        end: 892,
                                                        as_str(): "42",
                                                    },
                                                },
                                            },
                                            TyStructExpressionField {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 894,
                                                        end: 900,
                                                        as_str(): "second",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                value: TyExpression {
                                                    expression: EnumInstantiation {
                                                        enum_decl: TyEnumDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                    ),
                                                                    start: 18,
                                                                    end: 32,
                                                                    as_str(): "LowerLevelEnum",
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 39,
                                                                            end: 44,
                                                                            as_str(): "first",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_id: TypeId(
                                                                        0,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        0,
                                                                    ),
                                                                    type_span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 46,
                                                                        end: 50,
                                                                        as_str(): "b256",
                                                                    },
                                                                    tag: 0,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 39,
                                                                        end: 50,
                                                                        as_str(): "first: b256",
                                                                    },
                                                                    attributes: {},
                                                                },
                                                                TyEnumVariant {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 56,
                                                                            end: 62,
                                                                            as_str(): "second",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_id: TypeId(
                                                                        3,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        3,
                                                                    ),
                                                                    type_span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 64,
                                                                        end: 67,
                                                                        as_str(): "u32",
                                                                    },
                                                                    tag: 1,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 56,
                                                                        end: 67,
                                                                        as_str(): "second: u32",
                                                                    },
                                                                    attributes: {},
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 9,
                                                                end: 70,
                                                                as_str(): "pub enum LowerLevelEnum {\n    first: b256,\n    second: u32,\n}",
                                                            },
                                                            visibility: Public,
                                                        },
                                                        variant_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 62,
                                                                as_str(): "second",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        tag: 1,
                                                        contents: Some(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        66,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    17,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                    ),
                                                                    start: 925,
                                                                    end: 927,
                                                                    as_str(): "66",
                                                                },
                                                            },
                                                        ),
                                                        enum_instantiation_span: Span {
                                                            src (ptr): 0x00007fb0ee12f040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                            ),
                                                            start: 902,
                                                            end: 916,
                                                            as_str(): "LowerLevelEnum",
                                                        },
                                                        variant_instantiation_span: Span {
                                                            src (ptr): 0x00007fb0ee12f040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                            ),
                                                            start: 918,
                                                            end: 924,
                                                            as_str(): "second",
                                                        },
                                                        type_binding: TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 902,
                                                                end: 924,
                                                                as_str(): "LowerLevelEnum::second",
                                                            },
                                                        },
                                                    },
                                                    return_type: TypeId(
                                                        5,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 918,
                                                        end: 924,
                                                        as_str(): "second",
                                                    },
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 861,
                                            end: 872,
                                            as_str(): "ThenAStruct",
                                        },
                                    },
                                    return_type: TypeId(
                                        9,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 861,
                                        end: 934,
                                        as_str(): "ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    }",
                                    },
                                },
                            ),
                            enum_instantiation_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 840,
                                end: 852,
                                as_str(): "TopLevelEnum",
                            },
                            variant_instantiation_span: Span {
                                src (ptr): 0x00007fb0ee12f040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                ),
                                start: 854,
                                end: 860,
                                as_str(): "second",
                            },
                            type_binding: TypeBinding {
                                inner: (),
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fb0ee12f040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                    ),
                                    start: 840,
                                    end: 860,
                                    as_str(): "TopLevelEnum::second",
                                },
                            },
                        },
                        return_type: TypeId(
                            12,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 854,
                            end: 860,
                            as_str(): "second",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0ee12f040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                    ),
                    start: 840,
                    end: 935,
                    as_str(): "TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0ee12f040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
        ),
        start: 221,
        end: 937,
        as_str(): "fn main() -> TopLevelEnum {\n    // Expected output:\n    //\n    //  0000000000000001  # TopLevelEnum.tag\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  000000000000002a  #     ThenAStruct.first(42)\n    //  0000000000000001  #     ThenAStruct.LowerLevelEnum.tag\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000042  #         ThenAStruct.LowerLevelEnum.second(66)\n\n    TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })\n}",
    },
    attributes: {},
    return_type: TypeId(
        12,
    ),
    initial_return_type: TypeId(
        11,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb0ee12f040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
        ),
        start: 234,
        end: 246,
        as_str(): "TopLevelEnum",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

