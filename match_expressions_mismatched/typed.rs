TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a2c66060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
            ),
            start: 16,
            end: 24,
            as_str(): "MyStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 27,
                    end: 28,
                    as_str(): "a",
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
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 27,
                end: 33,
                as_str(): "a: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 30,
                end: 33,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 35,
                    end: 36,
                    as_str(): "b",
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
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 35,
                end: 41,
                as_str(): "b: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 38,
                end: 41,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0a2c66060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
        ),
        start: 9,
        end: 43,
        as_str(): "struct MyStruct { a: u64, b: u64 }",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a2c66060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
            ),
            start: 50,
            end: 56,
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
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 61,
                    end: 69,
                    as_str(): "Variant1",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31631,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 71,
                end: 73,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 61,
                end: 73,
                as_str(): "Variant1: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 77,
                    end: 85,
                    as_str(): "Variant2",
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
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 87,
                end: 90,
                as_str(): "u64",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 77,
                end: 90,
                as_str(): "Variant2: u64",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 94,
                    end: 102,
                    as_str(): "Variant3",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31633,
            ),
            initial_type_id: TypeId(
                31632,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 104,
                end: 112,
                as_str(): "MyStruct",
            },
            tag: 2,
            span: Span {
                src (ptr): 0x00007fe0a2c66060,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                ),
                start: 94,
                end: 112,
                as_str(): "Variant3: MyStruct",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0a2c66060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
        ),
        start: 45,
        end: 115,
        as_str(): "enum MyEnum {\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a2c66060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
            ),
            start: 120,
            end: 124,
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
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 142,
                                    end: 143,
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
                                                src (ptr): 0x00007fe0a2c66060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                ),
                                                start: 50,
                                                end: 56,
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
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 61,
                                                        end: 69,
                                                        as_str(): "Variant1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31631,
                                                ),
                                                initial_type_id: TypeId(
                                                    31630,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 73,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 73,
                                                    as_str(): "Variant1: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 77,
                                                        end: 85,
                                                        as_str(): "Variant2",
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
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 90,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 90,
                                                    as_str(): "Variant2: u64",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 94,
                                                        end: 102,
                                                        as_str(): "Variant3",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31633,
                                                ),
                                                initial_type_id: TypeId(
                                                    31632,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 112,
                                                    as_str(): "MyStruct",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 112,
                                                    as_str(): "Variant3: MyStruct",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 45,
                                            end: 115,
                                            as_str(): "enum MyEnum {\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 61,
                                            end: 69,
                                            as_str(): "Variant1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 146,
                                        end: 152,
                                        as_str(): "MyEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 162,
                                        as_str(): "Variant1",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 154,
                                            end: 162,
                                            as_str(): "Variant1",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31636,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 154,
                                    end: 162,
                                    as_str(): "Variant1",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31636,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 138,
                    end: 163,
                    as_str(): "let x = MyEnum::Variant1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 170,
                                    end: 171,
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
                                                src (ptr): 0x00007fe0a2c66060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                ),
                                                start: 50,
                                                end: 56,
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
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 61,
                                                        end: 69,
                                                        as_str(): "Variant1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31631,
                                                ),
                                                initial_type_id: TypeId(
                                                    31630,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 73,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 73,
                                                    as_str(): "Variant1: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 77,
                                                        end: 85,
                                                        as_str(): "Variant2",
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
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 90,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 90,
                                                    as_str(): "Variant2: u64",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 94,
                                                        end: 102,
                                                        as_str(): "Variant3",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31633,
                                                ),
                                                initial_type_id: TypeId(
                                                    31632,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 112,
                                                    as_str(): "MyStruct",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 112,
                                                    as_str(): "Variant3: MyStruct",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 45,
                                            end: 115,
                                            as_str(): "enum MyEnum {\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 85,
                                            as_str(): "Variant2",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
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
                                                src (ptr): 0x00007fe0a2c66060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                ),
                                                start: 193,
                                                end: 194,
                                                as_str(): "5",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 174,
                                        end: 180,
                                        as_str(): "MyEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 182,
                                        end: 190,
                                        as_str(): "Variant2",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 190,
                                            as_str(): "MyEnum::Variant2",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31636,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 182,
                                    end: 190,
                                    as_str(): "Variant2",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31636,
                            ),
                            type_ascription: TypeId(
                                31637,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 166,
                    end: 198,
                    as_str(): "let y = MyEnum::Variant2 ( 5 ) ;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 205,
                                    end: 206,
                                    as_str(): "z",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0a2c66060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                ),
                                                start: 50,
                                                end: 56,
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
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 61,
                                                        end: 69,
                                                        as_str(): "Variant1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31631,
                                                ),
                                                initial_type_id: TypeId(
                                                    31630,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 73,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 73,
                                                    as_str(): "Variant1: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 77,
                                                        end: 85,
                                                        as_str(): "Variant2",
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
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 90,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 90,
                                                    as_str(): "Variant2: u64",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 94,
                                                        end: 102,
                                                        as_str(): "Variant3",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31633,
                                                ),
                                                initial_type_id: TypeId(
                                                    31632,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 112,
                                                    as_str(): "MyStruct",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 112,
                                                    as_str(): "Variant3: MyStruct",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 45,
                                            end: 115,
                                            as_str(): "enum MyEnum {\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 94,
                                            end: 102,
                                            as_str(): "Variant3",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 2,
                                    contents: Some(
                                        TyExpression {
                                            expression: StructExpression {
                                                struct_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a2c66060,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                        ),
                                                        start: 16,
                                                        end: 24,
                                                        as_str(): "MyStruct",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                fields: [
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 239,
                                                                end: 240,
                                                                as_str(): "a",
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
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 242,
                                                                end: 243,
                                                                as_str(): "0",
                                                            },
                                                        },
                                                    },
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 245,
                                                                end: 246,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    1,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 248,
                                                                end: 249,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 228,
                                                    end: 236,
                                                    as_str(): "MyStruct",
                                                },
                                            },
                                            return_type: TypeId(
                                                31633,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0a2c66060,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                ),
                                                start: 228,
                                                end: 251,
                                                as_str(): "MyStruct { a: 0, b: 1 }",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 209,
                                        end: 215,
                                        as_str(): "MyEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 217,
                                        end: 225,
                                        as_str(): "Variant3",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 209,
                                            end: 225,
                                            as_str(): "MyEnum::Variant3",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31636,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 217,
                                    end: 225,
                                    as_str(): "Variant3",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31636,
                            ),
                            type_ascription: TypeId(
                                31640,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 201,
                    end: 255,
                    as_str(): "let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: CodeBlock(
                            TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: Declaration(
                                            VariableDeclaration(
                                                TyVariableDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "__match_return_var_name_1",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 170,
                                                                    end: 171,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 265,
                                                                end: 266,
                                                                as_str(): "y",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31636,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): "y",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31636,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31647,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 318,
                                            as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 266,
                                                                            as_str(): "y",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 266,
                                                                            as_str(): "y",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 265,
                                                                        end: 266,
                                                                        as_str(): "y",
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
                                                                            src (ptr): 0x00007fe0b26e3ba0,
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
                                                                                        name_override_opt: Some(
                                                                                            "__match_return_var_name_1",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                            ),
                                                                                            start: 265,
                                                                                            end: 266,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 266,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31636,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 266,
                                                                                    as_str(): "y",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 266,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b26e3ba0,
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
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 266,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13319,
                                                                Span {
                                                                    src (ptr): 0x00007fe0b26e3ba0,
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
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): "y",
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
                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                            ),
                                                                                            start: 292,
                                                                                            end: 293,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    body: TyExpression {
                                                                                        expression: UnsafeDowncast {
                                                                                            exp: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                            ),
                                                                                                            start: 265,
                                                                                                            end: 266,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                        ),
                                                                                                        start: 265,
                                                                                                        end: 266,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31636,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                    ),
                                                                                                    start: 265,
                                                                                                    end: 266,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                        ),
                                                                                                        start: 77,
                                                                                                        end: 85,
                                                                                                        as_str(): "Variant2",
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
                                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                    ),
                                                                                                    start: 87,
                                                                                                    end: 90,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                                tag: 1,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                    ),
                                                                                                    start: 77,
                                                                                                    end: 90,
                                                                                                    as_str(): "Variant2: u64",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                            ),
                                                                                            start: 273,
                                                                                            end: 295,
                                                                                            as_str(): "MyEnum::Variant2 ( y )",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 293,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                            ),
                                                                                            start: 292,
                                                                                            end: 293,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 299,
                                                                                        end: 300,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 299,
                                                                                    end: 300,
                                                                                    as_str(): "y",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 299,
                                                                            end: 300,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 300,
                                                            as_str(): "y",
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
                                                                                            10,
                                                                                        ),
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 311,
                                                                                        end: 313,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 311,
                                                                                end: 313,
                                                                                as_str(): "10",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 311,
                                                                end: 313,
                                                                as_str(): "10",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 299,
                                                    end: 300,
                                                    as_str(): "y",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 318,
                                            as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0a2c66060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                            ),
                            start: 259,
                            end: 318,
                            as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0a2c66060,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                    ),
                    start: 259,
                    end: 318,
                    as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0a2c66060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
        ),
        start: 117,
        end: 320,
        as_str(): "fn main() -> u64 {\n  let x = MyEnum::Variant1;\n  let y = MyEnum::Variant2 ( 5 ) ;\n  let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;\n\n  match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }\n}",
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
        src (ptr): 0x00007fe0a2c66060,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
        ),
        start: 130,
        end: 133,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

