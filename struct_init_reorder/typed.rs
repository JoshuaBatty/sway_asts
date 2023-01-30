
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 44,
            end: 57,
            as_str(): "MyInnerStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 64,
                    end: 65,
                    as_str(): "x",
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
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 64,
                end: 70,
                as_str(): "x: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 67,
                end: 70,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 76,
                    end: 77,
                    as_str(): "y",
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
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 76,
                end: 82,
                as_str(): "y: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 79,
                end: 82,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Public,
    span: Span {
        src (ptr): 0x00007fe03e568ce0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
        ),
        start: 33,
        end: 85,
        as_str(): "pub struct MyInnerStruct {\n    x: u64,\n    y: u64,\n}",
    },
    attributes: {},
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 98,
            end: 106,
            as_str(): "MyStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 115,
                    end: 120,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31631,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 115,
                end: 135,
                as_str(): "value: MyInnerStruct",
            },
            type_span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 122,
                end: 135,
                as_str(): "MyInnerStruct",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Public,
    span: Span {
        src (ptr): 0x00007fe03e568ce0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
        ),
        start: 87,
        end: 138,
        as_str(): "pub struct MyStruct {\n      value: MyInnerStruct,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 149,
            end: 155,
            as_str(): "MyEnum",
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
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 162,
                    end: 164,
                    as_str(): "V1",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                50,
            ),
            initial_type_id: TypeId(
                50,
            ),
            type_span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 166,
                end: 168,
                as_str(): "u8",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 162,
                end: 168,
                as_str(): "V1: u8",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 174,
                    end: 176,
                    as_str(): "V2",
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
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 178,
                end: 181,
                as_str(): "u64",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 174,
                end: 181,
                as_str(): "V2: u64",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe03e568ce0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
        ),
        start: 140,
        end: 184,
        as_str(): "pub enum MyEnum {\n    V1: u8,\n    V2: u64,\n}",
    },
    visibility: Public,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 197,
            end: 200,
            as_str(): "Foo",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 207,
                    end: 209,
                    as_str(): "f1",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31633,
            ),
            initial_type_id: TypeId(
                31632,
            ),
            span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 207,
                end: 217,
                as_str(): "f1: MyEnum",
            },
            type_span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 211,
                end: 217,
                as_str(): "MyEnum",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 223,
                    end: 225,
                    as_str(): "f2",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31635,
            ),
            initial_type_id: TypeId(
                31634,
            ),
            span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 223,
                end: 235,
                as_str(): "f2: MyStruct",
            },
            type_span: Span {
                src (ptr): 0x00007fe03e568ce0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                ),
                start: 227,
                end: 235,
                as_str(): "MyStruct",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Public,
    span: Span {
        src (ptr): 0x00007fe03e568ce0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
        ),
        start: 186,
        end: 238,
        as_str(): "pub struct Foo {\n    f1: MyEnum,\n    f2: MyStruct,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe03e568ce0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
            ),
            start: 243,
            end: 247,
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
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 260,
                                    end: 262,
                                    as_str(): "f1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe03e568ce0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                ),
                                                start: 149,
                                                end: 155,
                                                as_str(): "MyEnum",
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 162,
                                                        end: 164,
                                                        as_str(): "V1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    50,
                                                ),
                                                initial_type_id: TypeId(
                                                    50,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 168,
                                                    as_str(): "u8",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 162,
                                                    end: 168,
                                                    as_str(): "V1: u8",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 174,
                                                        end: 176,
                                                        as_str(): "V2",
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
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 178,
                                                    end: 181,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 181,
                                                    as_str(): "V2: u64",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 140,
                                            end: 184,
                                            as_str(): "pub enum MyEnum {\n    V1: u8,\n    V2: u64,\n}",
                                        },
                                        visibility: Public,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 162,
                                            end: 164,
                                            as_str(): "V1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U8(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe03e568ce0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                ),
                                                start: 285,
                                                end: 288,
                                                as_str(): "0u8",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 274,
                                        end: 280,
                                        as_str(): "MyEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 282,
                                        end: 284,
                                        as_str(): "V1",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 274,
                                            end: 284,
                                            as_str(): "MyEnum::V1",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31633,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 282,
                                    end: 284,
                                    as_str(): "V1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31633,
                            ),
                            type_ascription: TypeId(
                                31633,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 265,
                                    end: 271,
                                    as_str(): "MyEnum",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 256,
                    end: 290,
                    as_str(): "let f1 : MyEnum = MyEnum::V1(0u8);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 299,
                                    end: 301,
                                    as_str(): "f2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 106,
                                            as_str(): "MyStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 326,
                                                    end: 331,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: StructExpression {
                                                    struct_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 44,
                                                            end: 57,
                                                            as_str(): "MyInnerStruct",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    fields: [
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 349,
                                                                    end: 350,
                                                                    as_str(): "x",
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
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 352,
                                                                    end: 353,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 355,
                                                                    end: 356,
                                                                    as_str(): "y",
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
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 358,
                                                                    end: 359,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 333,
                                                        end: 346,
                                                        as_str(): "MyInnerStruct",
                                                    },
                                                },
                                                return_type: TypeId(
                                                    31631,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 333,
                                                    end: 361,
                                                    as_str(): "MyInnerStruct { x: 0, y: 0 }",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 315,
                                        end: 323,
                                        as_str(): "MyStruct",
                                    },
                                },
                                return_type: TypeId(
                                    31635,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 315,
                                    end: 363,
                                    as_str(): "MyStruct { value: MyInnerStruct { x: 0, y: 0 } }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31635,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe03e568ce0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                    ),
                                    start: 304,
                                    end: 312,
                                    as_str(): "MyStruct",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 295,
                    end: 364,
                    as_str(): "let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 452,
                                        end: 455,
                                        as_str(): "log",
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
                                            src (ptr): 0x00007fe04b255d60,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/logging.sw",
                                            ),
                                            start: 222,
                                            end: 227,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: StructExpression {
                                            struct_name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 197,
                                                    end: 200,
                                                    as_str(): "Foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            fields: [
                                                TyStructExpressionField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 482,
                                                            end: 484,
                                                            as_str(): "f1",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    value: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 260,
                                                                    end: 262,
                                                                    as_str(): "f1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 482,
                                                                end: 484,
                                                                as_str(): "f1",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31633,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 482,
                                                            end: 484,
                                                            as_str(): "f1",
                                                        },
                                                    },
                                                },
                                                TyStructExpressionField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 470,
                                                            end: 472,
                                                            as_str(): "f2",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    value: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 299,
                                                                    end: 301,
                                                                    as_str(): "f2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 470,
                                                                end: 472,
                                                                as_str(): "f2",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31635,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 470,
                                                            end: 472,
                                                            as_str(): "f2",
                                                        },
                                                    },
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fe03e568ce0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                ),
                                                start: 456,
                                                end: 459,
                                                as_str(): "Foo",
                                            },
                                        },
                                        return_type: TypeId(
                                            31653,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 456,
                                            end: 490,
                                            as_str(): "Foo {\n        f2,\n        f1\n    }",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13320,
                                Span {
                                    src (ptr): 0x00007fe04b255d60,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/logging.sw",
                                    ),
                                    start: 208,
                                    end: 258,
                                    as_str(): "pub fn log<T>(value: T) {\n    __log::<T>(value);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 452,
                                        end: 455,
                                        as_str(): "log",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31656,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 452,
                            end: 491,
                            as_str(): "log(Foo {\n        f2,\n        f1\n    })",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe03e568ce0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                    ),
                    start: 452,
                    end: 491,
                    as_str(): "log(Foo {\n        f2,\n        f1\n    })",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe03e568ce0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
        ),
        start: 240,
        end: 494,
        as_str(): "fn main() {\n    let f1 : MyEnum = MyEnum::V1(0u8);\n    let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };\n    // f1 and f2 are instantiated in the wrong order below. that shouldn't matter.\n    log(Foo {\n        f2,\n        f1\n    });\n}",
    },
    attributes: {},
    return_type: TypeId(
        31638,
    ),
    initial_return_type: TypeId(
        31637,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe03e568ce0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
        ),
        start: 240,
        end: 249,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

