TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 16,
            end: 24,
            as_str(): "Generic1",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 34,
                    end: 35,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7258,
            ),
            initial_type_id: TypeId(
                7259,
            ),
            span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 34,
                end: 38,
                as_str(): "a: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 37,
                end: 38,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7258),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fd5d5ae0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
        ),
        start: 9,
        end: 41,
        as_str(): "struct Generic1<T> {\n    a: T,\n}",
    },
    attributes: {},
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 50,
            end: 58,
            as_str(): "Generic2",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 68,
                    end: 69,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7262,
            ),
            initial_type_id: TypeId(
                7261,
            ),
            span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 68,
                end: 82,
                as_str(): "b: Generic1<T>",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 71,
                end: 82,
                as_str(): "Generic1<T>",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7260),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fd5d5ae0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
        ),
        start: 43,
        end: 85,
        as_str(): "struct Generic2<T> {\n    b: Generic1<T>,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 92,
            end: 100,
            as_str(): "Generic3",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7263),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 110,
                    end: 111,
                    as_str(): "A",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7263,
            ),
            initial_type_id: TypeId(
                7264,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 113,
                end: 114,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 110,
                end: 114,
                as_str(): "A: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 120,
                    end: 121,
                    as_str(): "B",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7263,
            ),
            initial_type_id: TypeId(
                7265,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 123,
                end: 124,
                as_str(): "T",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 120,
                end: 124,
                as_str(): "B: T",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fd5d5ae0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
        ),
        start: 87,
        end: 126,
        as_str(): "enum Generic3<T> {\n    A: T,\n    B: T\n}",
    },
    visibility: Private,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 133,
            end: 141,
            as_str(): "Generic4",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7266),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 151,
                    end: 152,
                    as_str(): "C",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7268,
            ),
            initial_type_id: TypeId(
                7267,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 154,
                end: 162,
                as_str(): "Generic3",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 151,
                end: 165,
                as_str(): "C: Generic3<T>",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 171,
                    end: 172,
                    as_str(): "D",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7270,
            ),
            initial_type_id: TypeId(
                7269,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 174,
                end: 182,
                as_str(): "Generic3",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0fd5d5ae0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                ),
                start: 171,
                end: 185,
                as_str(): "D: Generic3<T>",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fd5d5ae0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
        ),
        start: 128,
        end: 187,
        as_str(): "enum Generic4<T> {\n    C: Generic3<T>,\n    D: Generic3<T>\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd5d5ae0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
            ),
            start: 192,
            end: 196,
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
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 216,
                                    end: 217,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 24,
                                            as_str(): "Generic1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
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
                                                        7,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 242,
                                                    end: 246,
                                                    as_str(): "7u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 220,
                                        end: 228,
                                        as_str(): "Generic1",
                                    },
                                },
                                return_type: TypeId(
                                    7275,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 220,
                                    end: 252,
                                    as_str(): "Generic1 {\n        a: 7u64\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7275,
                            ),
                            type_ascription: TypeId(
                                7272,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 212,
                    end: 253,
                    as_str(): "let a = Generic1 {\n        a: 7u64\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 262,
                                    end: 263,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 58,
                                            as_str(): "Generic2",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 285,
                                                    end: 286,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 216,
                                                            end: 217,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 288,
                                                        end: 289,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7275,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 288,
                                                    end: 289,
                                                    as_str(): "a",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 266,
                                        end: 274,
                                        as_str(): "Generic2",
                                    },
                                },
                                return_type: TypeId(
                                    7281,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 266,
                                    end: 295,
                                    as_str(): "Generic2 {\n        b: a\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7281,
                            ),
                            type_ascription: TypeId(
                                7277,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 258,
                    end: 296,
                    as_str(): "let b = Generic2 {\n        b: a\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 305,
                                    end: 306,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 92,
                                                end: 100,
                                                as_str(): "Generic3",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(7284),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 110,
                                                        end: 111,
                                                        as_str(): "A",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7284,
                                                ),
                                                initial_type_id: TypeId(
                                                    7264,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 114,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 110,
                                                    end: 114,
                                                    as_str(): "A: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 120,
                                                        end: 121,
                                                        as_str(): "B",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7284,
                                                ),
                                                initial_type_id: TypeId(
                                                    7265,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 123,
                                                    end: 124,
                                                    as_str(): "T",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 120,
                                                    end: 124,
                                                    as_str(): "B: T",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 87,
                                            end: 126,
                                            as_str(): "enum Generic3<T> {\n    A: T,\n    B: T\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 120,
                                            end: 121,
                                            as_str(): "B",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
                                    contents: Some(
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 262,
                                                        end: 263,
                                                        as_str(): "b",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 321,
                                                    end: 322,
                                                    as_str(): "b",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                7281,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 321,
                                                end: 322,
                                                as_str(): "b",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 309,
                                        end: 317,
                                        as_str(): "Generic3",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 319,
                                        end: 320,
                                        as_str(): "B",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 309,
                                            end: 320,
                                            as_str(): "Generic3::B",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7287,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 319,
                                    end: 320,
                                    as_str(): "B",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7287,
                            ),
                            type_ascription: TypeId(
                                7283,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 301,
                    end: 324,
                    as_str(): "let c = Generic3::B(b);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 333,
                                    end: 334,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 133,
                                                end: 141,
                                                as_str(): "Generic4",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(7289),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 151,
                                                        end: 152,
                                                        as_str(): "C",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7290,
                                                ),
                                                initial_type_id: TypeId(
                                                    7267,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 154,
                                                    end: 162,
                                                    as_str(): "Generic3",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 151,
                                                    end: 165,
                                                    as_str(): "C: Generic3<T>",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 171,
                                                        end: 172,
                                                        as_str(): "D",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7291,
                                                ),
                                                initial_type_id: TypeId(
                                                    7269,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 182,
                                                    as_str(): "Generic3",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 171,
                                                    end: 185,
                                                    as_str(): "D: Generic3<T>",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 187,
                                            as_str(): "enum Generic4<T> {\n    C: Generic3<T>,\n    D: Generic3<T>\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 151,
                                            end: 152,
                                            as_str(): "C",
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
                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                        ),
                                                        start: 305,
                                                        end: 306,
                                                        as_str(): "c",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 349,
                                                    end: 350,
                                                    as_str(): "c",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                7287,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                ),
                                                start: 349,
                                                end: 350,
                                                as_str(): "c",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 337,
                                        end: 345,
                                        as_str(): "Generic4",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fd5d5ae0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                        ),
                                        start: 347,
                                        end: 348,
                                        as_str(): "C",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 337,
                                            end: 348,
                                            as_str(): "Generic4::C",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7294,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd5d5ae0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 348,
                                    as_str(): "C",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7294,
                            ),
                            type_ascription: TypeId(
                                7288,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 329,
                    end: 352,
                    as_str(): "let d = Generic4::C(c);",
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
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 365,
                                                            as_str(): "d",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 333,
                                                                    end: 334,
                                                                    as_str(): "d",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 364,
                                                                end: 365,
                                                                as_str(): "d",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7294,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 365,
                                                            as_str(): "d",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        7294,
                                                    ),
                                                    type_ascription: TypeId(
                                                        7295,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 358,
                                            end: 599,
                                            as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: LazyOperator {
                                                            op: And,
                                                            lhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 364,
                                                                                end: 365,
                                                                                as_str(): "d",
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
                                                                                    src (ptr): 0x00007fe0fd156b60,
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
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 364,
                                                                                                    end: 365,
                                                                                                    as_str(): "d",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 364,
                                                                                                end: 365,
                                                                                                as_str(): "d",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7294,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 364,
                                                                                            end: 365,
                                                                                            as_str(): "d",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): "d",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd156b60,
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
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): "d",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        551,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0fd156b60,
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
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 364,
                                                                    end: 365,
                                                                    as_str(): "d",
                                                                },
                                                            },
                                                            rhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 564,
                                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 564,
                                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 376,
                                                                                end: 564,
                                                                                as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
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
                                                                                    src (ptr): 0x00007fe0fd156b60,
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
                                                                                        expression: UnsafeDowncast {
                                                                                            exp: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 364,
                                                                                                            end: 365,
                                                                                                            as_str(): "d",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 364,
                                                                                                        end: 365,
                                                                                                        as_str(): "d",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7294,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 364,
                                                                                                    end: 365,
                                                                                                    as_str(): "d",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 151,
                                                                                                        end: 152,
                                                                                                        as_str(): "C",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    7298,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7267,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 154,
                                                                                                    end: 162,
                                                                                                    as_str(): "Generic3",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 151,
                                                                                                    end: 165,
                                                                                                    as_str(): "C: Generic3<T>",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7298,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 376,
                                                                                            end: 564,
                                                                                            as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 564,
                                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd156b60,
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
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 564,
                                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        550,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0fd156b60,
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
                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                    ),
                                                                    start: 376,
                                                                    end: 564,
                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 564,
                                                            as_str(): "d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
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
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 499,
                                                                                            end: 500,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    body: TyExpression {
                                                                                        expression: StructFieldAccess {
                                                                                            prefix: TyExpression {
                                                                                                expression: StructFieldAccess {
                                                                                                    prefix: TyExpression {
                                                                                                        expression: UnsafeDowncast {
                                                                                                            exp: TyExpression {
                                                                                                                expression: UnsafeDowncast {
                                                                                                                    exp: TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "__match_return_var_name_1",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 364,
                                                                                                                                    end: 365,
                                                                                                                                    as_str(): "d",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 364,
                                                                                                                                end: 365,
                                                                                                                                as_str(): "d",
                                                                                                                            },
                                                                                                                            mutability: Immutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            7294,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 364,
                                                                                                                            end: 365,
                                                                                                                            as_str(): "d",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    variant: TyEnumVariant {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 151,
                                                                                                                                end: 152,
                                                                                                                                as_str(): "C",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        type_id: TypeId(
                                                                                                                            7298,
                                                                                                                        ),
                                                                                                                        initial_type_id: TypeId(
                                                                                                                            7267,
                                                                                                                        ),
                                                                                                                        type_span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 154,
                                                                                                                            end: 162,
                                                                                                                            as_str(): "Generic3",
                                                                                                                        },
                                                                                                                        tag: 0,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 151,
                                                                                                                            end: 165,
                                                                                                                            as_str(): "C: Generic3<T>",
                                                                                                                        },
                                                                                                                        attributes: {},
                                                                                                                    },
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    7298,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 376,
                                                                                                                    end: 564,
                                                                                                                    as_str(): "Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        )",
                                                                                                                },
                                                                                                            },
                                                                                                            variant: TyEnumVariant {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 120,
                                                                                                                        end: 121,
                                                                                                                        as_str(): "B",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                type_id: TypeId(
                                                                                                                    7301,
                                                                                                                ),
                                                                                                                initial_type_id: TypeId(
                                                                                                                    7265,
                                                                                                                ),
                                                                                                                type_span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 123,
                                                                                                                    end: 124,
                                                                                                                    as_str(): "T",
                                                                                                                },
                                                                                                                tag: 1,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 120,
                                                                                                                    end: 124,
                                                                                                                    as_str(): "B: T",
                                                                                                                },
                                                                                                                attributes: {},
                                                                                                            },
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7301,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 401,
                                                                                                            end: 554,
                                                                                                            as_str(): "Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )",
                                                                                                        },
                                                                                                    },
                                                                                                    field_to_access: TyStructField {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 68,
                                                                                                                end: 69,
                                                                                                                as_str(): "b",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        type_id: TypeId(
                                                                                                            7280,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            7261,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 68,
                                                                                                            end: 82,
                                                                                                            as_str(): "b: Generic1<T>",
                                                                                                        },
                                                                                                        type_span: Span {
                                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 71,
                                                                                                            end: 82,
                                                                                                            as_str(): "Generic1<T>",
                                                                                                        },
                                                                                                        attributes: {},
                                                                                                    },
                                                                                                    field_instantiation_span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 461,
                                                                                                        end: 462,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                    resolved_type_of_parent: TypeId(
                                                                                                        7301,
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7280,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 461,
                                                                                                    end: 522,
                                                                                                    as_str(): "b: Generic1 {\n                        a\n                    }",
                                                                                                },
                                                                                            },
                                                                                            field_to_access: TyStructField {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 34,
                                                                                                        end: 35,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    7279,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7259,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 34,
                                                                                                    end: 38,
                                                                                                    as_str(): "a: T",
                                                                                                },
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 37,
                                                                                                    end: 38,
                                                                                                    as_str(): "T",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                            field_instantiation_span: Span {
                                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                                ),
                                                                                                start: 499,
                                                                                                end: 500,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            resolved_type_of_parent: TypeId(
                                                                                                7280,
                                                                                            ),
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7279,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 499,
                                                                                            end: 500,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        7279,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        7279,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 499,
                                                                            end: 500,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                            ),
                                                                                            start: 499,
                                                                                            end: 500,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 570,
                                                                                        end: 571,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7279,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                    ),
                                                                                    start: 570,
                                                                                    end: 571,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                            ),
                                                                            start: 570,
                                                                            end: 571,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            7279,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd5d5ae0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                            ),
                                                            start: 568,
                                                            end: 573,
                                                            as_str(): "{ a }",
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
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd5d5ae0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                        ),
                                                                                        start: 590,
                                                                                        end: 591,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                                ),
                                                                                start: 590,
                                                                                end: 591,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd5d5ae0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                                ),
                                                                start: 588,
                                                                end: 593,
                                                                as_str(): "{ 0 }",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    7279,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd5d5ae0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                                    ),
                                                    start: 568,
                                                    end: 573,
                                                    as_str(): "{ a }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd5d5ae0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                                            ),
                                            start: 358,
                                            end: 599,
                                            as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            7279,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd5d5ae0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                            ),
                            start: 358,
                            end: 599,
                            as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd5d5ae0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
                    ),
                    start: 358,
                    end: 599,
                    as_str(): "match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fd5d5ae0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
        ),
        start: 189,
        end: 601,
        as_str(): "fn main() -> u64 {\n    let a = Generic1 {\n        a: 7u64\n    };\n    let b = Generic2 {\n        b: a\n    };\n    let c = Generic3::B(b);\n    let d = Generic4::C(c);\n\n    match d {\n        Generic4::C(\n            Generic3::B(\n                Generic2 {\n                    b: Generic1 {\n                        a\n                    }\n                }\n            )\n        ) => { a },\n        _ => { 0 }\n    }\n}",
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
        src (ptr): 0x00007fe0fd5d5ae0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmaz5B3/generic_inside_generic/src/main.sw",
        ),
        start: 202,
        end: 205,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

