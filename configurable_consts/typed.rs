
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 40,
            end: 48,
            as_str(): "MyStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 55,
                    end: 56,
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
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 55,
                end: 61,
                as_str(): "x: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 58,
                end: 61,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 68,
                    end: 69,
                    as_str(): "y",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 68,
                end: 75,
                as_str(): "y: bool",
            },
            type_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 71,
                end: 75,
                as_str(): "bool",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 33,
        end: 77,
        as_str(): "struct MyStruct {\n    x: u64, \n    y: bool\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 84,
            end: 90,
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
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 97,
                    end: 98,
                    as_str(): "A",
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
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 100,
                end: 103,
                as_str(): "u64",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 97,
                end: 103,
                as_str(): "A: u64",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 109,
                    end: 110,
                    as_str(): "B",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 112,
                end: 116,
                as_str(): "bool",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 109,
                end: 116,
                as_str(): "B: bool",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 79,
        end: 119,
        as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        13340,
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 121,
            end: 411,
            as_str(): "impl core::ops::Eq for MyEnum {\n    fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }\n}",
        },
    ),
)
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 432,
            end: 434,
            as_str(): "C0",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: Literal(
            Boolean(
                true,
            ),
        ),
        return_type: TypeId(
            71,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 443,
            end: 447,
            as_str(): "true",
        },
    },
    visibility: Public,
    return_type: TypeId(
        71,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 436,
            end: 440,
            as_str(): "bool",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 432,
        end: 434,
        as_str(): "C0",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 453,
            end: 455,
            as_str(): "C1",
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
            31704,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 463,
            end: 465,
            as_str(): "42",
        },
    },
    visibility: Public,
    return_type: TypeId(
        21,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 457,
            end: 460,
            as_str(): "u64",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 453,
        end: 455,
        as_str(): "C1",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 471,
            end: 473,
            as_str(): "C2",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: Literal(
            B256(
                [
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                    17,
                ],
            ),
        ),
        return_type: TypeId(
            59,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 482,
            end: 548,
            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
        },
    },
    visibility: Public,
    return_type: TypeId(
        59,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 475,
            end: 479,
            as_str(): "b256",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 471,
        end: 473,
        as_str(): "C2",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 554,
            end: 556,
            as_str(): "C3",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: StructExpression {
            struct_name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 40,
                    end: 48,
                    as_str(): "MyStruct",
                },
                is_raw_ident: false,
            },
            fields: [
                TyStructExpressionField {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 580,
                            end: 581,
                            as_str(): "x",
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
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 583,
                            end: 585,
                            as_str(): "42",
                        },
                    },
                },
                TyStructExpressionField {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 587,
                            end: 588,
                            as_str(): "y",
                        },
                        is_raw_ident: false,
                    },
                    value: TyExpression {
                        expression: Literal(
                            Boolean(
                                true,
                            ),
                        ),
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 590,
                            end: 594,
                            as_str(): "true",
                        },
                    },
                },
            ],
            span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 569,
                end: 577,
                as_str(): "MyStruct",
            },
        },
        return_type: TypeId(
            31706,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 569,
            end: 596,
            as_str(): "MyStruct { x: 42, y: true }",
        },
    },
    visibility: Public,
    return_type: TypeId(
        31706,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 558,
            end: 566,
            as_str(): "MyStruct",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 554,
        end: 556,
        as_str(): "C3",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 602,
            end: 604,
            as_str(): "C4",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: EnumInstantiation {
            enum_decl: TyEnumDeclaration {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 84,
                        end: 90,
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
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 97,
                                end: 98,
                                as_str(): "A",
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
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 100,
                            end: 103,
                            as_str(): "u64",
                        },
                        tag: 0,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 97,
                            end: 103,
                            as_str(): "A: u64",
                        },
                        attributes: {},
                    },
                    TyEnumVariant {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 109,
                                end: 110,
                                as_str(): "B",
                            },
                            is_raw_ident: false,
                        },
                        type_id: TypeId(
                            71,
                        ),
                        initial_type_id: TypeId(
                            71,
                        ),
                        type_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 112,
                            end: 116,
                            as_str(): "bool",
                        },
                        tag: 1,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 109,
                            end: 116,
                            as_str(): "B: bool",
                        },
                        attributes: {},
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 79,
                    end: 119,
                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                },
                visibility: Private,
            },
            variant_name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 97,
                    end: 98,
                    as_str(): "A",
                },
                is_raw_ident: false,
            },
            tag: 0,
            contents: Some(
                TyExpression {
                    expression: Literal(
                        U64(
                            42,
                        ),
                    ),
                    return_type: TypeId(
                        21,
                    ),
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 625,
                        end: 627,
                        as_str(): "42",
                    },
                },
            ),
            enum_instantiation_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 615,
                end: 621,
                as_str(): "MyEnum",
            },
            variant_instantiation_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 623,
                end: 624,
                as_str(): "A",
            },
            type_binding: TypeBinding {
                inner: (),
                type_arguments: [],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 615,
                    end: 624,
                    as_str(): "MyEnum::A",
                },
            },
        },
        return_type: TypeId(
            31631,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 623,
            end: 624,
            as_str(): "A",
        },
    },
    visibility: Public,
    return_type: TypeId(
        31631,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 606,
            end: 612,
            as_str(): "MyEnum",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 602,
        end: 604,
        as_str(): "C4",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 634,
            end: 636,
            as_str(): "C5",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: EnumInstantiation {
            enum_decl: TyEnumDeclaration {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 84,
                        end: 90,
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
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 97,
                                end: 98,
                                as_str(): "A",
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
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 100,
                            end: 103,
                            as_str(): "u64",
                        },
                        tag: 0,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 97,
                            end: 103,
                            as_str(): "A: u64",
                        },
                        attributes: {},
                    },
                    TyEnumVariant {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 109,
                                end: 110,
                                as_str(): "B",
                            },
                            is_raw_ident: false,
                        },
                        type_id: TypeId(
                            71,
                        ),
                        initial_type_id: TypeId(
                            71,
                        ),
                        type_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 112,
                            end: 116,
                            as_str(): "bool",
                        },
                        tag: 1,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 109,
                            end: 116,
                            as_str(): "B: bool",
                        },
                        attributes: {},
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 79,
                    end: 119,
                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                },
                visibility: Private,
            },
            variant_name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 109,
                    end: 110,
                    as_str(): "B",
                },
                is_raw_ident: false,
            },
            tag: 1,
            contents: Some(
                TyExpression {
                    expression: Literal(
                        Boolean(
                            true,
                        ),
                    ),
                    return_type: TypeId(
                        71,
                    ),
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 657,
                        end: 661,
                        as_str(): "true",
                    },
                },
            ),
            enum_instantiation_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 647,
                end: 653,
                as_str(): "MyEnum",
            },
            variant_instantiation_span: Span {
                src (ptr): 0x00007fb11f3db040,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                ),
                start: 655,
                end: 656,
                as_str(): "B",
            },
            type_binding: TypeBinding {
                inner: (),
                type_arguments: [],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 647,
                    end: 656,
                    as_str(): "MyEnum::B",
                },
            },
        },
        return_type: TypeId(
            31631,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 655,
            end: 656,
            as_str(): "B",
        },
    },
    visibility: Public,
    return_type: TypeId(
        31631,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 638,
            end: 644,
            as_str(): "MyEnum",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 634,
        end: 636,
        as_str(): "C5",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 668,
            end: 670,
            as_str(): "C6",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: Literal(
            String(
                Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 682,
                    end: 686,
                    as_str(): "fuel",
                },
            ),
        ),
        return_type: TypeId(
            31717,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 681,
            end: 687,
            as_str(): "\"fuel\"",
        },
    },
    visibility: Public,
    return_type: TypeId(
        31717,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 672,
            end: 678,
            as_str(): "str[4]",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 668,
        end: 670,
        as_str(): "C6",
    },
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 693,
            end: 695,
            as_str(): "C7",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
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
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 709,
                        end: 710,
                        as_str(): "1",
                    },
                },
                TyExpression {
                    expression: Literal(
                        U64(
                            2,
                        ),
                    ),
                    return_type: TypeId(
                        21,
                    ),
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 712,
                        end: 713,
                        as_str(): "2",
                    },
                },
                TyExpression {
                    expression: Literal(
                        U64(
                            3,
                        ),
                    ),
                    return_type: TypeId(
                        21,
                    ),
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 715,
                        end: 716,
                        as_str(): "3",
                    },
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
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 718,
                        end: 719,
                        as_str(): "4",
                    },
                },
            ],
        },
        return_type: TypeId(
            31729,
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 708,
            end: 720,
            as_str(): "[1, 2, 3, 4]",
        },
    },
    visibility: Public,
    return_type: TypeId(
        31729,
    ),
    is_configurable: true,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 697,
            end: 705,
            as_str(): "[u64; 4]",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 693,
        end: 695,
        as_str(): "C7",
    },
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 745,
            end: 759,
            as_str(): "test_first_use",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 768,
                                        end: 774,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 778,
                                                            end: 780,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 778,
                                                            end: 780,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 778,
                                                        end: 780,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2930,
                                                            end: 2934,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 432,
                                                                    end: 434,
                                                                    as_str(): "C0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 775,
                                                                end: 777,
                                                                as_str(): "C0",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 775,
                                                            end: 777,
                                                            as_str(): "C0",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2936,
                                                            end: 2941,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 781,
                                                            end: 785,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13352,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2924,
                                                    end: 2990,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 778,
                                                        end: 780,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 775,
                                            end: 785,
                                            as_str(): "C0 == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13353,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 768,
                                        end: 774,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31737,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 768,
                            end: 786,
                            as_str(): "assert(C0 == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 768,
                    end: 786,
                    as_str(): "assert(C0 == true)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 792,
                                        end: 798,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 802,
                                                            end: 804,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 802,
                                                            end: 804,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 802,
                                                        end: 804,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 455,
                                                                    as_str(): "C1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 799,
                                                                end: 801,
                                                                as_str(): "C1",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 799,
                                                            end: 801,
                                                            as_str(): "C1",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 805,
                                                            end: 807,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13355,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 802,
                                                        end: 804,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 799,
                                            end: 807,
                                            as_str(): "C1 == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13356,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 792,
                                        end: 798,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31743,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 792,
                            end: 808,
                            as_str(): "assert(C1 == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 792,
                    end: 808,
                    as_str(): "assert(C1 == 42)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 814,
                                        end: 820,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 824,
                                                            end: 826,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 824,
                                                            end: 826,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 824,
                                                        end: 826,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 471,
                                                                    end: 473,
                                                                    as_str(): "C2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 821,
                                                                end: 823,
                                                                as_str(): "C2",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 821,
                                                            end: 823,
                                                            as_str(): "C2",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            B256(
                                                                [
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                ],
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 827,
                                                            end: 893,
                                                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13358,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 824,
                                                        end: 826,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 821,
                                            end: 893,
                                            as_str(): "C2 == 0x1111111111111111111111111111111111111111111111111111111111111111",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13359,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 814,
                                        end: 820,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31748,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 814,
                            end: 894,
                            as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 814,
                    end: 894,
                    as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 906,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 912,
                                                            end: 914,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 912,
                                                            end: 914,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 912,
                                                        end: 914,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "C3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 907,
                                                                        end: 909,
                                                                        as_str(): "C3",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31706,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 907,
                                                                    end: 909,
                                                                    as_str(): "C3",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 55,
                                                                        end: 56,
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
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 55,
                                                                    end: 61,
                                                                    as_str(): "x: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 61,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 910,
                                                                end: 911,
                                                                as_str(): "x",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31706,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 907,
                                                            end: 911,
                                                            as_str(): "C3.x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 915,
                                                            end: 917,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13361,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 912,
                                                        end: 914,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 907,
                                            end: 917,
                                            as_str(): "C3.x == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13362,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 906,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31755,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 900,
                            end: 918,
                            as_str(): "assert(C3.x == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 900,
                    end: 918,
                    as_str(): "assert(C3.x == 42)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 924,
                                        end: 930,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 936,
                                                            end: 938,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 936,
                                                            end: 938,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 936,
                                                        end: 938,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2930,
                                                            end: 2934,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "C3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 931,
                                                                        end: 933,
                                                                        as_str(): "C3",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31706,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 931,
                                                                    end: 933,
                                                                    as_str(): "C3",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    71,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 75,
                                                                    as_str(): "y: bool",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 75,
                                                                    as_str(): "bool",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 934,
                                                                end: 935,
                                                                as_str(): "y",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31706,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 931,
                                                            end: 935,
                                                            as_str(): "C3.y",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2936,
                                                            end: 2941,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 939,
                                                            end: 943,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13364,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2924,
                                                    end: 2990,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 936,
                                                        end: 938,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 931,
                                            end: 943,
                                            as_str(): "C3.y == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13365,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 924,
                                        end: 930,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31761,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 924,
                            end: 944,
                            as_str(): "assert(C3.y == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 924,
                    end: 944,
                    as_str(): "assert(C3.y == true)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 950,
                                        end: 956,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 960,
                                                            end: 962,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 960,
                                                            end: 962,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 960,
                                                        end: 962,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 167,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 602,
                                                                    end: 604,
                                                                    as_str(): "C4",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 957,
                                                                end: 959,
                                                                as_str(): "C4",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 957,
                                                            end: 959,
                                                            as_str(): "C4",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 90,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 97,
                                                                                end: 98,
                                                                                as_str(): "A",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
                                                                            as_str(): "u64",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 103,
                                                                            as_str(): "A: u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 116,
                                                                            as_str(): "B: bool",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 119,
                                                                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 97,
                                                                    end: 98,
                                                                    as_str(): "A",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 0,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 973,
                                                                        end: 975,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 963,
                                                                end: 969,
                                                                as_str(): "MyEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 971,
                                                                end: 972,
                                                                as_str(): "A",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 963,
                                                                    end: 972,
                                                                    as_str(): "MyEnum::A",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 971,
                                                            end: 972,
                                                            as_str(): "A",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13368,
                                                Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 409,
                                                    as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 960,
                                                        end: 962,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 957,
                                            end: 976,
                                            as_str(): "C4 == MyEnum::A(42)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13369,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 950,
                                        end: 956,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31768,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 950,
                            end: 977,
                            as_str(): "assert(C4 == MyEnum::A(42))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 950,
                    end: 977,
                    as_str(): "assert(C4 == MyEnum::A(42))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 983,
                                        end: 989,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 993,
                                                            end: 995,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 993,
                                                            end: 995,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 993,
                                                        end: 995,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 167,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 634,
                                                                    end: 636,
                                                                    as_str(): "C5",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 990,
                                                                end: 992,
                                                                as_str(): "C5",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 990,
                                                            end: 992,
                                                            as_str(): "C5",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 90,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 97,
                                                                                end: 98,
                                                                                as_str(): "A",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
                                                                            as_str(): "u64",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 103,
                                                                            as_str(): "A: u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 116,
                                                                            as_str(): "B: bool",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 119,
                                                                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 109,
                                                                    end: 110,
                                                                    as_str(): "B",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 1,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1006,
                                                                        end: 1010,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 996,
                                                                end: 1002,
                                                                as_str(): "MyEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1004,
                                                                end: 1005,
                                                                as_str(): "B",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 996,
                                                                    end: 1005,
                                                                    as_str(): "MyEnum::B",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1004,
                                                            end: 1005,
                                                            as_str(): "B",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13372,
                                                Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 409,
                                                    as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 993,
                                                        end: 995,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 990,
                                            end: 1011,
                                            as_str(): "C5 == MyEnum::B(true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13373,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 983,
                                        end: 989,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31774,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 983,
                            end: 1012,
                            as_str(): "assert(C5 == MyEnum::B(true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 983,
                    end: 1012,
                    as_str(): "assert(C5 == MyEnum::B(true))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1018,
                                        end: 1024,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1036,
                                                            end: 1038,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1036,
                                                            end: 1038,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1036,
                                                        end: 1038,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1025,
                                                                        end: 1031,
                                                                        as_str(): "sha256",
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
                                                                            src (ptr): 0x00007fb13a99fc60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 80,
                                                                            as_str(): "param",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 668,
                                                                                    end: 670,
                                                                                    as_str(): "C6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1032,
                                                                                end: 1034,
                                                                                as_str(): "C6",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31717,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1032,
                                                                            end: 1034,
                                                                            as_str(): "C6",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13378,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a99fc60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 926,
                                                                    as_str(): "pub fn sha256<T>(param: T) -> b256 {\n    let mut result_buffer: b256 = b256::min();\n    if !__is_reference_type::<T>() {\n        asm(buffer, ptr: param, eight_bytes: 8, hash: result_buffer) {\n            move buffer sp; // Make `buffer` point to the current top of the stack\n            cfei i8; // Grow stack by 1 word\n            sw buffer ptr i0; // Save value in register at \"ptr\" to memory at \"buffer\"\n            s256 hash buffer eight_bytes; // Hash the next eight bytes starting from \"buffer\" into \"hash\"\n            cfsi i8; // Shrink stack by 1 word\n            hash: b256 // Return\n        }\n    } else {\n        let size = __size_of::<T>();\n        asm(hash: result_buffer, ptr: param, bytes: size) {\n            s256 hash ptr bytes; // Hash the next \"size\" number of bytes starting from \"ptr\" into \"hash\"\n            hash: b256 // Return\n        }\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1025,
                                                                        end: 1031,
                                                                        as_str(): "sha256",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1025,
                                                            end: 1035,
                                                            as_str(): "sha256(C6)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1039,
                                                                        end: 1045,
                                                                        as_str(): "sha256",
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
                                                                            src (ptr): 0x00007fb13a99fc60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 80,
                                                                            as_str(): "param",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            String(
                                                                                Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1047,
                                                                                    end: 1051,
                                                                                    as_str(): "fuel",
                                                                                },
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31783,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1046,
                                                                            end: 1052,
                                                                            as_str(): "\"fuel\"",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13382,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a99fc60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 926,
                                                                    as_str(): "pub fn sha256<T>(param: T) -> b256 {\n    let mut result_buffer: b256 = b256::min();\n    if !__is_reference_type::<T>() {\n        asm(buffer, ptr: param, eight_bytes: 8, hash: result_buffer) {\n            move buffer sp; // Make `buffer` point to the current top of the stack\n            cfei i8; // Grow stack by 1 word\n            sw buffer ptr i0; // Save value in register at \"ptr\" to memory at \"buffer\"\n            s256 hash buffer eight_bytes; // Hash the next eight bytes starting from \"buffer\" into \"hash\"\n            cfsi i8; // Shrink stack by 1 word\n            hash: b256 // Return\n        }\n    } else {\n        let size = __size_of::<T>();\n        asm(hash: result_buffer, ptr: param, bytes: size) {\n            s256 hash ptr bytes; // Hash the next \"size\" number of bytes starting from \"ptr\" into \"hash\"\n            hash: b256 // Return\n        }\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1039,
                                                                        end: 1045,
                                                                        as_str(): "sha256",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1039,
                                                            end: 1053,
                                                            as_str(): "sha256(\"fuel\")",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13383,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1036,
                                                        end: 1038,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1025,
                                            end: 1053,
                                            as_str(): "sha256(C6) == sha256(\"fuel\")",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13384,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1018,
                                        end: 1024,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31784,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1018,
                            end: 1054,
                            as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1018,
                    end: 1054,
                    as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1060,
                                        end: 1066,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1073,
                                                            end: 1075,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1073,
                                                            end: 1075,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1073,
                                                        end: 1075,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1067,
                                                                        end: 1069,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31790,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1067,
                                                                    end: 1069,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        0,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31791,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1070,
                                                                    end: 1071,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1067,
                                                            end: 1072,
                                                            as_str(): "C7[0]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1076,
                                                            end: 1077,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13386,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1073,
                                                        end: 1075,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1067,
                                            end: 1077,
                                            as_str(): "C7[0] == 1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13387,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1060,
                                        end: 1066,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31794,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1060,
                            end: 1078,
                            as_str(): "assert(C7[0] == 1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1060,
                    end: 1078,
                    as_str(): "assert(C7[0] == 1)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1084,
                                        end: 1090,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1097,
                                                            end: 1099,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1097,
                                                            end: 1099,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1097,
                                                        end: 1099,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1091,
                                                                        end: 1093,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31800,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1091,
                                                                    end: 1093,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        1,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31801,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1094,
                                                                    end: 1095,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1091,
                                                            end: 1096,
                                                            as_str(): "C7[1]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1100,
                                                            end: 1101,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13389,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1097,
                                                        end: 1099,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1091,
                                            end: 1101,
                                            as_str(): "C7[1] == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13390,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1084,
                                        end: 1090,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31804,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1084,
                            end: 1102,
                            as_str(): "assert(C7[1] == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1084,
                    end: 1102,
                    as_str(): "assert(C7[1] == 2)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1108,
                                        end: 1114,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1121,
                                                            end: 1123,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1121,
                                                            end: 1123,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1121,
                                                        end: 1123,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1115,
                                                                        end: 1117,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31810,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1115,
                                                                    end: 1117,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        2,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31811,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1118,
                                                                    end: 1119,
                                                                    as_str(): "2",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1115,
                                                            end: 1120,
                                                            as_str(): "C7[2]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1124,
                                                            end: 1125,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13392,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1121,
                                                        end: 1123,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1115,
                                            end: 1125,
                                            as_str(): "C7[2] == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13393,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1108,
                                        end: 1114,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31814,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1108,
                            end: 1126,
                            as_str(): "assert(C7[2] == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1108,
                    end: 1126,
                    as_str(): "assert(C7[2] == 3)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1132,
                                        end: 1138,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1145,
                                                            end: 1147,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1145,
                                                            end: 1147,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1145,
                                                        end: 1147,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1139,
                                                                        end: 1141,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31820,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1139,
                                                                    end: 1141,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        3,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31821,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1142,
                                                                    end: 1143,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1139,
                                                            end: 1144,
                                                            as_str(): "C7[3]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                4,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1148,
                                                            end: 1149,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13395,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1145,
                                                        end: 1147,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1139,
                                            end: 1149,
                                            as_str(): "C7[3] == 4",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13396,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1132,
                                        end: 1138,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31824,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1132,
                            end: 1150,
                            as_str(): "assert(C7[3] == 4)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1132,
                    end: 1150,
                    as_str(): "assert(C7[3] == 4)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 742,
        end: 1153,
        as_str(): "fn test_first_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
    },
    attributes: {
        Inline: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 727,
                        end: 733,
                        as_str(): "inline",
                    },
                    is_raw_ident: false,
                },
                args: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 734,
                            end: 739,
                            as_str(): "never",
                        },
                        is_raw_ident: false,
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 725,
                    end: 741,
                    as_str(): "#[inline(never)]",
                },
            },
        ],
    },
    return_type: TypeId(
        31732,
    ),
    initial_return_type: TypeId(
        31731,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 742,
        end: 761,
        as_str(): "fn test_first_use()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 1175,
            end: 1190,
            as_str(): "test_second_use",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1199,
                                        end: 1205,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1209,
                                                            end: 1211,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1209,
                                                            end: 1211,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1209,
                                                        end: 1211,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2930,
                                                            end: 2934,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 432,
                                                                    end: 434,
                                                                    as_str(): "C0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1206,
                                                                end: 1208,
                                                                as_str(): "C0",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1206,
                                                            end: 1208,
                                                            as_str(): "C0",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2936,
                                                            end: 2941,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1212,
                                                            end: 1216,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13399,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2924,
                                                    end: 2990,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1209,
                                                        end: 1211,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1206,
                                            end: 1216,
                                            as_str(): "C0 == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13400,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1199,
                                        end: 1205,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31833,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1199,
                            end: 1217,
                            as_str(): "assert(C0 == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1199,
                    end: 1217,
                    as_str(): "assert(C0 == true)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1223,
                                        end: 1229,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1233,
                                                            end: 1235,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1233,
                                                            end: 1235,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1233,
                                                        end: 1235,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 455,
                                                                    as_str(): "C1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1230,
                                                                end: 1232,
                                                                as_str(): "C1",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1230,
                                                            end: 1232,
                                                            as_str(): "C1",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1236,
                                                            end: 1238,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13402,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1233,
                                                        end: 1235,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1230,
                                            end: 1238,
                                            as_str(): "C1 == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13403,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1223,
                                        end: 1229,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31839,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1223,
                            end: 1239,
                            as_str(): "assert(C1 == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1223,
                    end: 1239,
                    as_str(): "assert(C1 == 42)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1245,
                                        end: 1251,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1255,
                                                            end: 1257,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1255,
                                                            end: 1257,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1255,
                                                        end: 1257,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 471,
                                                                    end: 473,
                                                                    as_str(): "C2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1252,
                                                                end: 1254,
                                                                as_str(): "C2",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1252,
                                                            end: 1254,
                                                            as_str(): "C2",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            B256(
                                                                [
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                ],
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1258,
                                                            end: 1324,
                                                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13405,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1255,
                                                        end: 1257,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1252,
                                            end: 1324,
                                            as_str(): "C2 == 0x1111111111111111111111111111111111111111111111111111111111111111",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13406,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1245,
                                        end: 1251,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31844,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1245,
                            end: 1325,
                            as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1245,
                    end: 1325,
                    as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1331,
                                        end: 1337,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1343,
                                                            end: 1345,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1343,
                                                            end: 1345,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1343,
                                                        end: 1345,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "C3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1338,
                                                                        end: 1340,
                                                                        as_str(): "C3",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31706,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1338,
                                                                    end: 1340,
                                                                    as_str(): "C3",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 55,
                                                                        end: 56,
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
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 55,
                                                                    end: 61,
                                                                    as_str(): "x: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 61,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1341,
                                                                end: 1342,
                                                                as_str(): "x",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31706,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1338,
                                                            end: 1342,
                                                            as_str(): "C3.x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1346,
                                                            end: 1348,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13408,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1343,
                                                        end: 1345,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1338,
                                            end: 1348,
                                            as_str(): "C3.x == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13409,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1331,
                                        end: 1337,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31851,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1331,
                            end: 1349,
                            as_str(): "assert(C3.x == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1331,
                    end: 1349,
                    as_str(): "assert(C3.x == 42)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1355,
                                        end: 1361,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1367,
                                                            end: 1369,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1367,
                                                            end: 1369,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1367,
                                                        end: 1369,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2930,
                                                            end: 2934,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "C3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1362,
                                                                        end: 1364,
                                                                        as_str(): "C3",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31706,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1362,
                                                                    end: 1364,
                                                                    as_str(): "C3",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    71,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 75,
                                                                    as_str(): "y: bool",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 75,
                                                                    as_str(): "bool",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1365,
                                                                end: 1366,
                                                                as_str(): "y",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31706,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1362,
                                                            end: 1366,
                                                            as_str(): "C3.y",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2936,
                                                            end: 2941,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1370,
                                                            end: 1374,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13411,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2924,
                                                    end: 2990,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1367,
                                                        end: 1369,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1362,
                                            end: 1374,
                                            as_str(): "C3.y == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13412,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1355,
                                        end: 1361,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31857,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1355,
                            end: 1375,
                            as_str(): "assert(C3.y == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1355,
                    end: 1375,
                    as_str(): "assert(C3.y == true)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1381,
                                        end: 1387,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1391,
                                                            end: 1393,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1391,
                                                            end: 1393,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1391,
                                                        end: 1393,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 167,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 602,
                                                                    end: 604,
                                                                    as_str(): "C4",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1388,
                                                                end: 1390,
                                                                as_str(): "C4",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1388,
                                                            end: 1390,
                                                            as_str(): "C4",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 90,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 97,
                                                                                end: 98,
                                                                                as_str(): "A",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
                                                                            as_str(): "u64",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 103,
                                                                            as_str(): "A: u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 116,
                                                                            as_str(): "B: bool",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 119,
                                                                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 97,
                                                                    end: 98,
                                                                    as_str(): "A",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 0,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1404,
                                                                        end: 1406,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1394,
                                                                end: 1400,
                                                                as_str(): "MyEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1402,
                                                                end: 1403,
                                                                as_str(): "A",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1394,
                                                                    end: 1403,
                                                                    as_str(): "MyEnum::A",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1402,
                                                            end: 1403,
                                                            as_str(): "A",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13415,
                                                Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 409,
                                                    as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1391,
                                                        end: 1393,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1388,
                                            end: 1407,
                                            as_str(): "C4 == MyEnum::A(42)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13416,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1381,
                                        end: 1387,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31864,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1381,
                            end: 1408,
                            as_str(): "assert(C4 == MyEnum::A(42))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1381,
                    end: 1408,
                    as_str(): "assert(C4 == MyEnum::A(42))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1414,
                                        end: 1420,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1424,
                                                            end: 1426,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1424,
                                                            end: 1426,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1424,
                                                        end: 1426,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 167,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 634,
                                                                    end: 636,
                                                                    as_str(): "C5",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1421,
                                                                end: 1423,
                                                                as_str(): "C5",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1421,
                                                            end: 1423,
                                                            as_str(): "C5",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 90,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 97,
                                                                                end: 98,
                                                                                as_str(): "A",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
                                                                            as_str(): "u64",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 103,
                                                                            as_str(): "A: u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 116,
                                                                            as_str(): "B: bool",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 119,
                                                                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 109,
                                                                    end: 110,
                                                                    as_str(): "B",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 1,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1437,
                                                                        end: 1441,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1427,
                                                                end: 1433,
                                                                as_str(): "MyEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1435,
                                                                end: 1436,
                                                                as_str(): "B",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1427,
                                                                    end: 1436,
                                                                    as_str(): "MyEnum::B",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1435,
                                                            end: 1436,
                                                            as_str(): "B",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13419,
                                                Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 409,
                                                    as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1424,
                                                        end: 1426,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1421,
                                            end: 1442,
                                            as_str(): "C5 == MyEnum::B(true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13420,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1414,
                                        end: 1420,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31870,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1414,
                            end: 1443,
                            as_str(): "assert(C5 == MyEnum::B(true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1414,
                    end: 1443,
                    as_str(): "assert(C5 == MyEnum::B(true))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1449,
                                        end: 1455,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1467,
                                                            end: 1469,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1467,
                                                            end: 1469,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1467,
                                                        end: 1469,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1456,
                                                                        end: 1462,
                                                                        as_str(): "sha256",
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
                                                                            src (ptr): 0x00007fb13a99fc60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 80,
                                                                            as_str(): "param",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 668,
                                                                                    end: 670,
                                                                                    as_str(): "C6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1463,
                                                                                end: 1465,
                                                                                as_str(): "C6",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31717,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1463,
                                                                            end: 1465,
                                                                            as_str(): "C6",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13425,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a99fc60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 926,
                                                                    as_str(): "pub fn sha256<T>(param: T) -> b256 {\n    let mut result_buffer: b256 = b256::min();\n    if !__is_reference_type::<T>() {\n        asm(buffer, ptr: param, eight_bytes: 8, hash: result_buffer) {\n            move buffer sp; // Make `buffer` point to the current top of the stack\n            cfei i8; // Grow stack by 1 word\n            sw buffer ptr i0; // Save value in register at \"ptr\" to memory at \"buffer\"\n            s256 hash buffer eight_bytes; // Hash the next eight bytes starting from \"buffer\" into \"hash\"\n            cfsi i8; // Shrink stack by 1 word\n            hash: b256 // Return\n        }\n    } else {\n        let size = __size_of::<T>();\n        asm(hash: result_buffer, ptr: param, bytes: size) {\n            s256 hash ptr bytes; // Hash the next \"size\" number of bytes starting from \"ptr\" into \"hash\"\n            hash: b256 // Return\n        }\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1456,
                                                                        end: 1462,
                                                                        as_str(): "sha256",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1456,
                                                            end: 1466,
                                                            as_str(): "sha256(C6)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1470,
                                                                        end: 1476,
                                                                        as_str(): "sha256",
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
                                                                            src (ptr): 0x00007fb13a99fc60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 80,
                                                                            as_str(): "param",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            String(
                                                                                Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1478,
                                                                                    end: 1482,
                                                                                    as_str(): "fuel",
                                                                                },
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31879,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1477,
                                                                            end: 1483,
                                                                            as_str(): "\"fuel\"",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13429,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a99fc60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 926,
                                                                    as_str(): "pub fn sha256<T>(param: T) -> b256 {\n    let mut result_buffer: b256 = b256::min();\n    if !__is_reference_type::<T>() {\n        asm(buffer, ptr: param, eight_bytes: 8, hash: result_buffer) {\n            move buffer sp; // Make `buffer` point to the current top of the stack\n            cfei i8; // Grow stack by 1 word\n            sw buffer ptr i0; // Save value in register at \"ptr\" to memory at \"buffer\"\n            s256 hash buffer eight_bytes; // Hash the next eight bytes starting from \"buffer\" into \"hash\"\n            cfsi i8; // Shrink stack by 1 word\n            hash: b256 // Return\n        }\n    } else {\n        let size = __size_of::<T>();\n        asm(hash: result_buffer, ptr: param, bytes: size) {\n            s256 hash ptr bytes; // Hash the next \"size\" number of bytes starting from \"ptr\" into \"hash\"\n            hash: b256 // Return\n        }\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1470,
                                                                        end: 1476,
                                                                        as_str(): "sha256",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1470,
                                                            end: 1484,
                                                            as_str(): "sha256(\"fuel\")",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13430,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1467,
                                                        end: 1469,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1456,
                                            end: 1484,
                                            as_str(): "sha256(C6) == sha256(\"fuel\")",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13431,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1449,
                                        end: 1455,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31880,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1449,
                            end: 1485,
                            as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1449,
                    end: 1485,
                    as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1491,
                                        end: 1497,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1504,
                                                            end: 1506,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1504,
                                                            end: 1506,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1504,
                                                        end: 1506,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1498,
                                                                        end: 1500,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31886,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1498,
                                                                    end: 1500,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        0,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31887,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1501,
                                                                    end: 1502,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1498,
                                                            end: 1503,
                                                            as_str(): "C7[0]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1507,
                                                            end: 1508,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13433,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1504,
                                                        end: 1506,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1498,
                                            end: 1508,
                                            as_str(): "C7[0] == 1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13434,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1491,
                                        end: 1497,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31890,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1491,
                            end: 1509,
                            as_str(): "assert(C7[0] == 1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1491,
                    end: 1509,
                    as_str(): "assert(C7[0] == 1)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1515,
                                        end: 1521,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1528,
                                                            end: 1530,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1528,
                                                            end: 1530,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1528,
                                                        end: 1530,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1522,
                                                                        end: 1524,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31896,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1522,
                                                                    end: 1524,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        1,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31897,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1525,
                                                                    end: 1526,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1522,
                                                            end: 1527,
                                                            as_str(): "C7[1]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1531,
                                                            end: 1532,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13436,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1528,
                                                        end: 1530,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1522,
                                            end: 1532,
                                            as_str(): "C7[1] == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13437,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1515,
                                        end: 1521,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31900,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1515,
                            end: 1533,
                            as_str(): "assert(C7[1] == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1515,
                    end: 1533,
                    as_str(): "assert(C7[1] == 2)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1539,
                                        end: 1545,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1552,
                                                            end: 1554,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1552,
                                                            end: 1554,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1552,
                                                        end: 1554,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1546,
                                                                        end: 1548,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31906,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1546,
                                                                    end: 1548,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        2,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31907,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1549,
                                                                    end: 1550,
                                                                    as_str(): "2",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1546,
                                                            end: 1551,
                                                            as_str(): "C7[2]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1555,
                                                            end: 1556,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13439,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1552,
                                                        end: 1554,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1546,
                                            end: 1556,
                                            as_str(): "C7[2] == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13440,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1539,
                                        end: 1545,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31910,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1539,
                            end: 1557,
                            as_str(): "assert(C7[2] == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1539,
                    end: 1557,
                    as_str(): "assert(C7[2] == 3)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1563,
                                        end: 1569,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1576,
                                                            end: 1578,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1576,
                                                            end: 1578,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1576,
                                                        end: 1578,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1570,
                                                                        end: 1572,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31916,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1570,
                                                                    end: 1572,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        3,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31917,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1573,
                                                                    end: 1574,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1570,
                                                            end: 1575,
                                                            as_str(): "C7[3]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                4,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1579,
                                                            end: 1580,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13442,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1576,
                                                        end: 1578,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1570,
                                            end: 1580,
                                            as_str(): "C7[3] == 4",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13443,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1563,
                                        end: 1569,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31920,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1563,
                            end: 1581,
                            as_str(): "assert(C7[3] == 4)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1563,
                    end: 1581,
                    as_str(): "assert(C7[3] == 4)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 1172,
        end: 1584,
        as_str(): "fn test_second_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
    },
    attributes: {
        Inline: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 1157,
                        end: 1163,
                        as_str(): "inline",
                    },
                    is_raw_ident: false,
                },
                args: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1164,
                            end: 1169,
                            as_str(): "never",
                        },
                        is_raw_ident: false,
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1155,
                    end: 1171,
                    as_str(): "#[inline(never)]",
                },
            },
        ],
    },
    return_type: TypeId(
        31828,
    ),
    initial_return_type: TypeId(
        31827,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 1172,
        end: 1192,
        as_str(): "fn test_second_use()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 1607,
            end: 1622,
            as_str(): "test_inline_use",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1631,
                                        end: 1637,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1641,
                                                            end: 1643,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1641,
                                                            end: 1643,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1641,
                                                        end: 1643,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2930,
                                                            end: 2934,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 432,
                                                                    end: 434,
                                                                    as_str(): "C0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1638,
                                                                end: 1640,
                                                                as_str(): "C0",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1638,
                                                            end: 1640,
                                                            as_str(): "C0",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2936,
                                                            end: 2941,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1644,
                                                            end: 1648,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13446,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2924,
                                                    end: 2990,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1641,
                                                        end: 1643,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1638,
                                            end: 1648,
                                            as_str(): "C0 == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13447,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1631,
                                        end: 1637,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31929,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1631,
                            end: 1649,
                            as_str(): "assert(C0 == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1631,
                    end: 1649,
                    as_str(): "assert(C0 == true)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1655,
                                        end: 1661,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1665,
                                                            end: 1667,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1665,
                                                            end: 1667,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1665,
                                                        end: 1667,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 455,
                                                                    as_str(): "C1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1662,
                                                                end: 1664,
                                                                as_str(): "C1",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1662,
                                                            end: 1664,
                                                            as_str(): "C1",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1668,
                                                            end: 1670,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13449,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1665,
                                                        end: 1667,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1662,
                                            end: 1670,
                                            as_str(): "C1 == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13450,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1655,
                                        end: 1661,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31935,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1655,
                            end: 1671,
                            as_str(): "assert(C1 == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1655,
                    end: 1671,
                    as_str(): "assert(C1 == 42)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1677,
                                        end: 1683,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1687,
                                                            end: 1689,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1687,
                                                            end: 1689,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1687,
                                                        end: 1689,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 471,
                                                                    end: 473,
                                                                    as_str(): "C2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1684,
                                                                end: 1686,
                                                                as_str(): "C2",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1684,
                                                            end: 1686,
                                                            as_str(): "C2",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            B256(
                                                                [
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                    17,
                                                                ],
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1690,
                                                            end: 1756,
                                                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13452,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1687,
                                                        end: 1689,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1684,
                                            end: 1756,
                                            as_str(): "C2 == 0x1111111111111111111111111111111111111111111111111111111111111111",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13453,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1677,
                                        end: 1683,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31940,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1677,
                            end: 1757,
                            as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1677,
                    end: 1757,
                    as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1763,
                                        end: 1769,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1775,
                                                            end: 1777,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1775,
                                                            end: 1777,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1775,
                                                        end: 1777,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "C3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1770,
                                                                        end: 1772,
                                                                        as_str(): "C3",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31706,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1770,
                                                                    end: 1772,
                                                                    as_str(): "C3",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 55,
                                                                        end: 56,
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
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 55,
                                                                    end: 61,
                                                                    as_str(): "x: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 61,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1773,
                                                                end: 1774,
                                                                as_str(): "x",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31706,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1770,
                                                            end: 1774,
                                                            as_str(): "C3.x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1778,
                                                            end: 1780,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13455,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1775,
                                                        end: 1777,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1770,
                                            end: 1780,
                                            as_str(): "C3.x == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13456,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1763,
                                        end: 1769,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31947,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1763,
                            end: 1781,
                            as_str(): "assert(C3.x == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1763,
                    end: 1781,
                    as_str(): "assert(C3.x == 42)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1787,
                                        end: 1793,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1799,
                                                            end: 1801,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1799,
                                                            end: 1801,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1799,
                                                        end: 1801,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2930,
                                                            end: 2934,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "C3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1794,
                                                                        end: 1796,
                                                                        as_str(): "C3",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31706,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1794,
                                                                    end: 1796,
                                                                    as_str(): "C3",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 69,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    71,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 75,
                                                                    as_str(): "y: bool",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 75,
                                                                    as_str(): "bool",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1797,
                                                                end: 1798,
                                                                as_str(): "y",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31706,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1794,
                                                            end: 1798,
                                                            as_str(): "C3.y",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2936,
                                                            end: 2941,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1802,
                                                            end: 1806,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13458,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2924,
                                                    end: 2990,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1799,
                                                        end: 1801,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1794,
                                            end: 1806,
                                            as_str(): "C3.y == true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13459,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1787,
                                        end: 1793,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31953,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1787,
                            end: 1807,
                            as_str(): "assert(C3.y == true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1787,
                    end: 1807,
                    as_str(): "assert(C3.y == true)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1813,
                                        end: 1819,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1823,
                                                            end: 1825,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1823,
                                                            end: 1825,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1823,
                                                        end: 1825,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 167,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 602,
                                                                    end: 604,
                                                                    as_str(): "C4",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1820,
                                                                end: 1822,
                                                                as_str(): "C4",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1820,
                                                            end: 1822,
                                                            as_str(): "C4",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 90,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 97,
                                                                                end: 98,
                                                                                as_str(): "A",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
                                                                            as_str(): "u64",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 103,
                                                                            as_str(): "A: u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 116,
                                                                            as_str(): "B: bool",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 119,
                                                                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 97,
                                                                    end: 98,
                                                                    as_str(): "A",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 0,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1836,
                                                                        end: 1838,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1826,
                                                                end: 1832,
                                                                as_str(): "MyEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1834,
                                                                end: 1835,
                                                                as_str(): "A",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1826,
                                                                    end: 1835,
                                                                    as_str(): "MyEnum::A",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1834,
                                                            end: 1835,
                                                            as_str(): "A",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13462,
                                                Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 409,
                                                    as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1823,
                                                        end: 1825,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1820,
                                            end: 1839,
                                            as_str(): "C4 == MyEnum::A(42)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13463,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1813,
                                        end: 1819,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31960,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1813,
                            end: 1840,
                            as_str(): "assert(C4 == MyEnum::A(42))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1813,
                    end: 1840,
                    as_str(): "assert(C4 == MyEnum::A(42))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1846,
                                        end: 1852,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1856,
                                                            end: 1858,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1856,
                                                            end: 1858,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1856,
                                                        end: 1858,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 167,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 634,
                                                                    end: 636,
                                                                    as_str(): "C5",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1853,
                                                                end: 1855,
                                                                as_str(): "C5",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1853,
                                                            end: 1855,
                                                            as_str(): "C5",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 84,
                                                                        end: 90,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 97,
                                                                                end: 98,
                                                                                as_str(): "A",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
                                                                            as_str(): "u64",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 103,
                                                                            as_str(): "A: u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "B",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            71,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
                                                                        },
                                                                        tag: 1,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 116,
                                                                            as_str(): "B: bool",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 119,
                                                                    as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 109,
                                                                    end: 110,
                                                                    as_str(): "B",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 1,
                                                            contents: Some(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1869,
                                                                        end: 1873,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                            ),
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1859,
                                                                end: 1865,
                                                                as_str(): "MyEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1867,
                                                                end: 1868,
                                                                as_str(): "B",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1859,
                                                                    end: 1868,
                                                                    as_str(): "MyEnum::B",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            31631,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1867,
                                                            end: 1868,
                                                            as_str(): "B",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13466,
                                                Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 409,
                                                    as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1856,
                                                        end: 1858,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1853,
                                            end: 1874,
                                            as_str(): "C5 == MyEnum::B(true)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13467,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1846,
                                        end: 1852,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31966,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1846,
                            end: 1875,
                            as_str(): "assert(C5 == MyEnum::B(true))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1846,
                    end: 1875,
                    as_str(): "assert(C5 == MyEnum::B(true))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1881,
                                        end: 1887,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1899,
                                                            end: 1901,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1899,
                                                            end: 1901,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1899,
                                                        end: 1901,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3390,
                                                            end: 3394,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1888,
                                                                        end: 1894,
                                                                        as_str(): "sha256",
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
                                                                            src (ptr): 0x00007fb13a99fc60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 80,
                                                                            as_str(): "param",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 668,
                                                                                    end: 670,
                                                                                    as_str(): "C6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1895,
                                                                                end: 1897,
                                                                                as_str(): "C6",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31717,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1895,
                                                                            end: 1897,
                                                                            as_str(): "C6",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13472,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a99fc60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 926,
                                                                    as_str(): "pub fn sha256<T>(param: T) -> b256 {\n    let mut result_buffer: b256 = b256::min();\n    if !__is_reference_type::<T>() {\n        asm(buffer, ptr: param, eight_bytes: 8, hash: result_buffer) {\n            move buffer sp; // Make `buffer` point to the current top of the stack\n            cfei i8; // Grow stack by 1 word\n            sw buffer ptr i0; // Save value in register at \"ptr\" to memory at \"buffer\"\n            s256 hash buffer eight_bytes; // Hash the next eight bytes starting from \"buffer\" into \"hash\"\n            cfsi i8; // Shrink stack by 1 word\n            hash: b256 // Return\n        }\n    } else {\n        let size = __size_of::<T>();\n        asm(hash: result_buffer, ptr: param, bytes: size) {\n            s256 hash ptr bytes; // Hash the next \"size\" number of bytes starting from \"ptr\" into \"hash\"\n            hash: b256 // Return\n        }\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1888,
                                                                        end: 1894,
                                                                        as_str(): "sha256",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1888,
                                                            end: 1898,
                                                            as_str(): "sha256(C6)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3396,
                                                            end: 3401,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1902,
                                                                        end: 1908,
                                                                        as_str(): "sha256",
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
                                                                            src (ptr): 0x00007fb13a99fc60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 80,
                                                                            as_str(): "param",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            String(
                                                                                Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1910,
                                                                                    end: 1914,
                                                                                    as_str(): "fuel",
                                                                                },
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31975,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1909,
                                                                            end: 1915,
                                                                            as_str(): "\"fuel\"",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13476,
                                                                Span {
                                                                    src (ptr): 0x00007fb13a99fc60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/hash.sw",
                                                                    ),
                                                                    start: 58,
                                                                    end: 926,
                                                                    as_str(): "pub fn sha256<T>(param: T) -> b256 {\n    let mut result_buffer: b256 = b256::min();\n    if !__is_reference_type::<T>() {\n        asm(buffer, ptr: param, eight_bytes: 8, hash: result_buffer) {\n            move buffer sp; // Make `buffer` point to the current top of the stack\n            cfei i8; // Grow stack by 1 word\n            sw buffer ptr i0; // Save value in register at \"ptr\" to memory at \"buffer\"\n            s256 hash buffer eight_bytes; // Hash the next eight bytes starting from \"buffer\" into \"hash\"\n            cfsi i8; // Shrink stack by 1 word\n            hash: b256 // Return\n        }\n    } else {\n        let size = __size_of::<T>();\n        asm(hash: result_buffer, ptr: param, bytes: size) {\n            s256 hash ptr bytes; // Hash the next \"size\" number of bytes starting from \"ptr\" into \"hash\"\n            hash: b256 // Return\n        }\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1902,
                                                                        end: 1908,
                                                                        as_str(): "sha256",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            59,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1902,
                                                            end: 1916,
                                                            as_str(): "sha256(\"fuel\")",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13477,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3384,
                                                    end: 3636,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1899,
                                                        end: 1901,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1888,
                                            end: 1916,
                                            as_str(): "sha256(C6) == sha256(\"fuel\")",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13478,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1881,
                                        end: 1887,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31976,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1881,
                            end: 1917,
                            as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1881,
                    end: 1917,
                    as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1923,
                                        end: 1929,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1936,
                                                            end: 1938,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1936,
                                                            end: 1938,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1936,
                                                        end: 1938,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1930,
                                                                        end: 1932,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31982,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1930,
                                                                    end: 1932,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        0,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31983,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1933,
                                                                    end: 1934,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1930,
                                                            end: 1935,
                                                            as_str(): "C7[0]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1939,
                                                            end: 1940,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13480,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1936,
                                                        end: 1938,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1930,
                                            end: 1940,
                                            as_str(): "C7[0] == 1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13481,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1923,
                                        end: 1929,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31986,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1923,
                            end: 1941,
                            as_str(): "assert(C7[0] == 1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1923,
                    end: 1941,
                    as_str(): "assert(C7[0] == 1)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1947,
                                        end: 1953,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1960,
                                                            end: 1962,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1960,
                                                            end: 1962,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1960,
                                                        end: 1962,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1954,
                                                                        end: 1956,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31992,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1954,
                                                                    end: 1956,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        1,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    31993,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1957,
                                                                    end: 1958,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1954,
                                                            end: 1959,
                                                            as_str(): "C7[1]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1963,
                                                            end: 1964,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13483,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1960,
                                                        end: 1962,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1954,
                                            end: 1964,
                                            as_str(): "C7[1] == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13484,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1947,
                                        end: 1953,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31996,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1947,
                            end: 1965,
                            as_str(): "assert(C7[1] == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1947,
                    end: 1965,
                    as_str(): "assert(C7[1] == 2)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1971,
                                        end: 1977,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1984,
                                                            end: 1986,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1984,
                                                            end: 1986,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1984,
                                                        end: 1986,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1978,
                                                                        end: 1980,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    32002,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1978,
                                                                    end: 1980,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        2,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    32003,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 1981,
                                                                    end: 1982,
                                                                    as_str(): "2",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1978,
                                                            end: 1983,
                                                            as_str(): "C7[2]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                3,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1987,
                                                            end: 1988,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13486,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1984,
                                                        end: 1986,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1978,
                                            end: 1988,
                                            as_str(): "C7[2] == 3",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13487,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1971,
                                        end: 1977,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32006,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1971,
                            end: 1989,
                            as_str(): "assert(C7[2] == 3)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1971,
                    end: 1989,
                    as_str(): "assert(C7[2] == 3)",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1995,
                                        end: 2001,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fb1273666d0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2008,
                                                            end: 2010,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2008,
                                                            end: 2010,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2008,
                                                        end: 2010,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fb12754f190,
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
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "C7",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2002,
                                                                        end: 2004,
                                                                        as_str(): "C7",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    32012,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 2002,
                                                                    end: 2004,
                                                                    as_str(): "C7",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        3,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    32013,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 2005,
                                                                    end: 2006,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2002,
                                                            end: 2007,
                                                            as_str(): "C7[3]",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12754f190,
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
                                                                4,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2011,
                                                            end: 2012,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13489,
                                                Span {
                                                    src (ptr): 0x00007fb12754f190,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2008,
                                                        end: 2010,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2002,
                                            end: 2012,
                                            as_str(): "C7[3] == 4",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13490,
                                Span {
                                    src (ptr): 0x00007fb1273666d0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1995,
                                        end: 2001,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32016,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1995,
                            end: 2013,
                            as_str(): "assert(C7[3] == 4)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1995,
                    end: 2013,
                    as_str(): "assert(C7[3] == 4)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 1604,
        end: 2016,
        as_str(): "fn test_inline_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
    },
    attributes: {
        Inline: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 1588,
                        end: 1594,
                        as_str(): "inline",
                    },
                    is_raw_ident: false,
                },
                args: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1595,
                            end: 1601,
                            as_str(): "always",
                        },
                        is_raw_ident: false,
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 1586,
                    end: 1603,
                    as_str(): "#[inline(always)]",
                },
            },
        ],
    },
    return_type: TypeId(
        31924,
    ),
    initial_return_type: TypeId(
        31923,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 1604,
        end: 1624,
        as_str(): "fn test_inline_use()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 2038,
            end: 2055,
            as_str(): "test_various_uses",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2064,
                                        end: 2078,
                                        as_str(): "test_first_use",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13493,
                                Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 742,
                                    end: 1153,
                                    as_str(): "fn test_first_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2064,
                                        end: 2078,
                                        as_str(): "test_first_use",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32022,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2064,
                            end: 2080,
                            as_str(): "test_first_use()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 2064,
                    end: 2080,
                    as_str(): "test_first_use()",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2086,
                                        end: 2101,
                                        as_str(): "test_second_use",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13495,
                                Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1172,
                                    end: 1584,
                                    as_str(): "fn test_second_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2086,
                                        end: 2101,
                                        as_str(): "test_second_use",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32024,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2086,
                            end: 2103,
                            as_str(): "test_second_use()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 2086,
                    end: 2103,
                    as_str(): "test_second_use()",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2109,
                                        end: 2124,
                                        as_str(): "test_inline_use",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13497,
                                Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1604,
                                    end: 2016,
                                    as_str(): "fn test_inline_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2109,
                                        end: 2124,
                                        as_str(): "test_inline_use",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32026,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2109,
                            end: 2126,
                            as_str(): "test_inline_use()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 2109,
                    end: 2126,
                    as_str(): "test_inline_use()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 2035,
        end: 2129,
        as_str(): "fn test_various_uses() {\n    test_first_use();\n    test_second_use();\n    test_inline_use();\n}",
    },
    attributes: {
        Inline: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 2020,
                        end: 2026,
                        as_str(): "inline",
                    },
                    is_raw_ident: false,
                },
                args: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2027,
                            end: 2032,
                            as_str(): "never",
                        },
                        is_raw_ident: false,
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 2018,
                    end: 2034,
                    as_str(): "#[inline(never)]",
                },
            },
        ],
    },
    return_type: TypeId(
        32020,
    ),
    initial_return_type: TypeId(
        32019,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 2035,
        end: 2057,
        as_str(): "fn test_various_uses()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 2134,
            end: 2138,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2147,
                                        end: 2164,
                                        as_str(): "test_various_uses",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13500,
                                Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 2035,
                                    end: 2129,
                                    as_str(): "fn test_various_uses() {\n    test_first_use();\n    test_second_use();\n    test_inline_use();\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2147,
                                        end: 2164,
                                        as_str(): "test_various_uses",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32032,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2147,
                            end: 2166,
                            as_str(): "test_various_uses()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb11f3db040,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                    ),
                    start: 2147,
                    end: 2166,
                    as_str(): "test_various_uses()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 2131,
        end: 2169,
        as_str(): "fn main() {\n    test_various_uses();\n}",
    },
    attributes: {},
    return_type: TypeId(
        32030,
    ),
    initial_return_type: TypeId(
        32029,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb11f3db040,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
        ),
        start: 2131,
        end: 2140,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

