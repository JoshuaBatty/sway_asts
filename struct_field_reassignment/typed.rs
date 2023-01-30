TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe056c4b800,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
            ),
            start: 251,
            end: 265,
            as_str(): "NumberOrString",
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
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 272,
                    end: 278,
                    as_str(): "Number",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 280,
                end: 283,
                as_str(): "u64",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 272,
                end: 283,
                as_str(): "Number: u64",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 289,
                    end: 295,
                    as_str(): "String",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7251,
            ),
            initial_type_id: TypeId(
                7251,
            ),
            type_span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 297,
                end: 303,
                as_str(): "str[4]",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 289,
                end: 303,
                as_str(): "String: str[4]",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe056c4b800,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
        ),
        start: 246,
        end: 306,
        as_str(): "enum NumberOrString {\n    Number: u64,\n    String: str[4],\n}",
    },
    visibility: Private,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe056c4b800,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
            ),
            start: 315,
            end: 319,
            as_str(): "Data",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 326,
                    end: 331,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7253,
            ),
            initial_type_id: TypeId(
                7252,
            ),
            span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 326,
                end: 347,
                as_str(): "value: NumberOrString",
            },
            type_span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 333,
                end: 347,
                as_str(): "NumberOrString",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 353,
                    end: 360,
                    as_str(): "address",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                50,
            ),
            initial_type_id: TypeId(
                50,
            ),
            span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 353,
                end: 364,
                as_str(): "address: u8",
            },
            type_span: Span {
                src (ptr): 0x00007fe056c4b800,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                ),
                start: 362,
                end: 364,
                as_str(): "u8",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe056c4b800,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
        ),
        start: 308,
        end: 367,
        as_str(): "struct Data {\n    value: NumberOrString,\n    address: u8,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe056c4b800,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
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
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 85,
                                    end: 89,
                                    as_str(): "data",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 315,
                                            end: 319,
                                            as_str(): "Data",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 107,
                                                    end: 112,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: EnumInstantiation {
                                                    enum_decl: TyEnumDeclaration {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 251,
                                                                end: 265,
                                                                as_str(): "NumberOrString",
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
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 272,
                                                                        end: 278,
                                                                        as_str(): "Number",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    21,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 280,
                                                                    end: 283,
                                                                    as_str(): "u64",
                                                                },
                                                                tag: 0,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 272,
                                                                    end: 283,
                                                                    as_str(): "Number: u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            TyEnumVariant {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 289,
                                                                        end: 295,
                                                                        as_str(): "String",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    7251,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    7251,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 297,
                                                                    end: 303,
                                                                    as_str(): "str[4]",
                                                                },
                                                                tag: 1,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 289,
                                                                    end: 303,
                                                                    as_str(): "String: str[4]",
                                                                },
                                                                attributes: {},
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 246,
                                                            end: 306,
                                                            as_str(): "enum NumberOrString {\n    Number: u64,\n    String: str[4],\n}",
                                                        },
                                                        visibility: Private,
                                                    },
                                                    variant_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 278,
                                                            as_str(): "Number",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    tag: 0,
                                                    contents: Some(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    20,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 137,
                                                                end: 139,
                                                                as_str(): "20",
                                                            },
                                                        },
                                                    ),
                                                    enum_instantiation_span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 114,
                                                        end: 128,
                                                        as_str(): "NumberOrString",
                                                    },
                                                    variant_instantiation_span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 130,
                                                        end: 136,
                                                        as_str(): "Number",
                                                    },
                                                    type_binding: TypeBinding {
                                                        inner: (),
                                                        type_arguments: [],
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 136,
                                                            as_str(): "NumberOrString::Number",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    7253,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 130,
                                                    end: 136,
                                                    as_str(): "Number",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 150,
                                                    end: 157,
                                                    as_str(): "address",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        15,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 159,
                                                    end: 169,
                                                    as_str(): "0b00001111",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 96,
                                        as_str(): "Data",
                                    },
                                },
                                return_type: TypeId(
                                    7257,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 92,
                                    end: 176,
                                    as_str(): "Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    }",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                7257,
                            ),
                            type_ascription: TypeId(
                                7255,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 77,
                    end: 177,
                    as_str(): "let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };",
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
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 183,
                                        end: 187,
                                        as_str(): "data",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7257,
                                ),
                                lhs_indices: [
                                    StructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe056c4b800,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                ),
                                                start: 188,
                                                end: 193,
                                                as_str(): "value",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: EnumInstantiation {
                                        enum_decl: TyEnumDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 251,
                                                    end: 265,
                                                    as_str(): "NumberOrString",
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
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 278,
                                                            as_str(): "Number",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_id: TypeId(
                                                        21,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        21,
                                                    ),
                                                    type_span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 280,
                                                        end: 283,
                                                        as_str(): "u64",
                                                    },
                                                    tag: 0,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 272,
                                                        end: 283,
                                                        as_str(): "Number: u64",
                                                    },
                                                    attributes: {},
                                                },
                                                TyEnumVariant {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 295,
                                                            as_str(): "String",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_id: TypeId(
                                                        7251,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        7251,
                                                    ),
                                                    type_span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 297,
                                                        end: 303,
                                                        as_str(): "str[4]",
                                                    },
                                                    tag: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 289,
                                                        end: 303,
                                                        as_str(): "String: str[4]",
                                                    },
                                                    attributes: {},
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fe056c4b800,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                ),
                                                start: 246,
                                                end: 306,
                                                as_str(): "enum NumberOrString {\n    Number: u64,\n    String: str[4],\n}",
                                            },
                                            visibility: Private,
                                        },
                                        variant_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe056c4b800,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                ),
                                                start: 289,
                                                end: 295,
                                                as_str(): "String",
                                            },
                                            is_raw_ident: false,
                                        },
                                        tag: 1,
                                        contents: Some(
                                            TyExpression {
                                                expression: Literal(
                                                    String(
                                                        Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 225,
                                                            as_str(): "sway",
                                                        },
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    7265,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 220,
                                                    end: 226,
                                                    as_str(): "\"sway\"",
                                                },
                                            },
                                        ),
                                        enum_instantiation_span: Span {
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 196,
                                            end: 210,
                                            as_str(): "NumberOrString",
                                        },
                                        variant_instantiation_span: Span {
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 212,
                                            end: 218,
                                            as_str(): "String",
                                        },
                                        type_binding: TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe056c4b800,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                ),
                                                start: 196,
                                                end: 218,
                                                as_str(): "NumberOrString::String",
                                            },
                                        },
                                    },
                                    return_type: TypeId(
                                        7253,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 212,
                                        end: 218,
                                        as_str(): "String",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7267,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 183,
                            end: 227,
                            as_str(): "data.value = NumberOrString::String( \"sway\")",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 183,
                    end: 227,
                    as_str(): "data.value = NumberOrString::String( \"sway\")",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
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
                                    src (ptr): 0x00007fe056c4b800,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                    ),
                                    start: 240,
                                    end: 241,
                                    as_str(): "0",
                                },
                            },
                        ),
                        return_type: TypeId(
                            7271,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 233,
                            end: 241,
                            as_str(): "return 0",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe056c4b800,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                    ),
                    start: 233,
                    end: 241,
                    as_str(): "return 0",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe056c4b800,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
        ),
        start: 54,
        end: 244,
        as_str(): "fn main() -> u64 {\n    let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };\n\n    data.value = NumberOrString::String( \"sway\");\n    return 0;\n}",
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
        src (ptr): 0x00007fe056c4b800,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
        ),
        start: 67,
        end: 70,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

