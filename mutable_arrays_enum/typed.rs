TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0935f24b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
            ),
            start: 16,
            end: 17,
            as_str(): "X",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0935f24b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                    ),
                    start: 24,
                    end: 29,
                    as_str(): "value",
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
                src (ptr): 0x00007fe0935f24b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                ),
                start: 24,
                end: 34,
                as_str(): "value: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0935f24b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                ),
                start: 31,
                end: 34,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0935f24b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
        ),
        start: 9,
        end: 36,
        as_str(): "struct X {\n    value: u64\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0935f24b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
            ),
            start: 43,
            end: 46,
            as_str(): "Foo",
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
                    src (ptr): 0x00007fe0935f24b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                    ),
                    start: 53,
                    end: 56,
                    as_str(): "Bar",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7253,
            ),
            initial_type_id: TypeId(
                7252,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0935f24b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                ),
                start: 58,
                end: 59,
                as_str(): "X",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0935f24b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                ),
                start: 53,
                end: 59,
                as_str(): "Bar: X",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0935f24b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
        ),
        start: 38,
        end: 62,
        as_str(): "enum Foo {\n    Bar: X,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0935f24b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
            ),
            start: 67,
            end: 71,
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
                                    src (ptr): 0x00007fe0935f24b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 103,
                                    as_str(): "my_array",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: EnumInstantiation {
                                                enum_decl: TyEnumDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 43,
                                                            end: 46,
                                                            as_str(): "Foo",
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
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 53,
                                                                    end: 56,
                                                                    as_str(): "Bar",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            type_id: TypeId(
                                                                7253,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                7252,
                                                            ),
                                                            type_span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 58,
                                                                end: 59,
                                                                as_str(): "X",
                                                            },
                                                            tag: 0,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 59,
                                                                as_str(): "Bar: X",
                                                            },
                                                            attributes: {},
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 38,
                                                        end: 62,
                                                        as_str(): "enum Foo {\n    Bar: X,\n}",
                                                    },
                                                    visibility: Private,
                                                },
                                                variant_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 53,
                                                        end: 56,
                                                        as_str(): "Bar",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                tag: 0,
                                                contents: Some(
                                                    TyExpression {
                                                        expression: StructExpression {
                                                            struct_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 16,
                                                                    end: 17,
                                                                    as_str(): "X",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            fields: [
                                                                TyStructExpressionField {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 128,
                                                                            end: 133,
                                                                            as_str(): "value",
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
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 135,
                                                                            end: 137,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 126,
                                                                end: 127,
                                                                as_str(): "X",
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7253,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 126,
                                                            end: 138,
                                                            as_str(): "X{value: 10}",
                                                        },
                                                    },
                                                ),
                                                enum_instantiation_span: Span {
                                                    src (ptr): 0x00007fe0935f24b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                    ),
                                                    start: 117,
                                                    end: 120,
                                                    as_str(): "Foo",
                                                },
                                                variant_instantiation_span: Span {
                                                    src (ptr): 0x00007fe0935f24b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 125,
                                                    as_str(): "Bar",
                                                },
                                                type_binding: TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 117,
                                                        end: 125,
                                                        as_str(): "Foo::Bar",
                                                    },
                                                },
                                            },
                                            return_type: TypeId(
                                                7256,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0935f24b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                ),
                                                start: 122,
                                                end: 125,
                                                as_str(): "Bar",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7264,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0935f24b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                    ),
                                    start: 116,
                                    end: 140,
                                    as_str(): "[Foo::Bar(X{value: 10})]",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                7264,
                            ),
                            type_ascription: TypeId(
                                7257,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0935f24b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 113,
                                    as_str(): "[Foo; 1]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0935f24b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                    ),
                    start: 87,
                    end: 141,
                    as_str(): "let mut my_array: [Foo; 1] = [Foo::Bar(X{value: 10})];",
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
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 146,
                                        end: 154,
                                        as_str(): "my_array",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7264,
                                ),
                                lhs_indices: [
                                    ArrayIndex {
                                        index: TyExpression {
                                            expression: Literal(
                                                U64(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0935f24b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                ),
                                                start: 155,
                                                end: 156,
                                                as_str(): "0",
                                            },
                                        },
                                        index_span: Span {
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 155,
                                            end: 156,
                                            as_str(): "0",
                                        },
                                    },
                                ],
                                rhs: TyExpression {
                                    expression: EnumInstantiation {
                                        enum_decl: TyEnumDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0935f24b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                    ),
                                                    start: 43,
                                                    end: 46,
                                                    as_str(): "Foo",
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
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 56,
                                                            as_str(): "Bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_id: TypeId(
                                                        7253,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        7252,
                                                    ),
                                                    type_span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 58,
                                                        end: 59,
                                                        as_str(): "X",
                                                    },
                                                    tag: 0,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 53,
                                                        end: 59,
                                                        as_str(): "Bar: X",
                                                    },
                                                    attributes: {},
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fe0935f24b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                ),
                                                start: 38,
                                                end: 62,
                                                as_str(): "enum Foo {\n    Bar: X,\n}",
                                            },
                                            visibility: Private,
                                        },
                                        variant_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0935f24b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                ),
                                                start: 53,
                                                end: 56,
                                                as_str(): "Bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        tag: 0,
                                        contents: Some(
                                            TyExpression {
                                                expression: StructExpression {
                                                    struct_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 16,
                                                            end: 17,
                                                            as_str(): "X",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    fields: [
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 171,
                                                                    end: 176,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        20,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 180,
                                                                    as_str(): "20",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 170,
                                                        as_str(): "X",
                                                    },
                                                },
                                                return_type: TypeId(
                                                    7253,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0935f24b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 181,
                                                    as_str(): "X{value: 20}",
                                                },
                                            },
                                        ),
                                        enum_instantiation_span: Span {
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 160,
                                            end: 163,
                                            as_str(): "Foo",
                                        },
                                        variant_instantiation_span: Span {
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 165,
                                            end: 168,
                                            as_str(): "Bar",
                                        },
                                        type_binding: TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0935f24b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                ),
                                                start: 160,
                                                end: 168,
                                                as_str(): "Foo::Bar",
                                            },
                                        },
                                    },
                                    return_type: TypeId(
                                        7256,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 165,
                                        end: 168,
                                        as_str(): "Bar",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7273,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0935f24b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                            ),
                            start: 146,
                            end: 182,
                            as_str(): "my_array[0] = Foo::Bar(X{value: 20})",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0935f24b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                    ),
                    start: 146,
                    end: 182,
                    as_str(): "my_array[0] = Foo::Bar(X{value: 20})",
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
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 205,
                                                            as_str(): "my_array[0]",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: ArrayIndex {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 95,
                                                                            end: 103,
                                                                            as_str(): "my_array",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 202,
                                                                        as_str(): "my_array",
                                                                    },
                                                                    mutability: Mutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7277,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 202,
                                                                    as_str(): "my_array",
                                                                },
                                                            },
                                                            index: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        0,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    7278,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 204,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7256,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 205,
                                                            as_str(): "my_array[0]",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        7256,
                                                    ),
                                                    type_ascription: TypeId(
                                                        7274,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 188,
                                            end: 245,
                                            as_str(): "match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }",
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
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 205,
                                                                            as_str(): "my_array[0]",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 205,
                                                                            as_str(): "my_array[0]",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 205,
                                                                        as_str(): "my_array[0]",
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
                                                                            src (ptr): 0x00007fe09298f300,
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
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 194,
                                                                                            end: 205,
                                                                                            as_str(): "my_array[0]",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 205,
                                                                                        as_str(): "my_array[0]",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7256,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 194,
                                                                                    end: 205,
                                                                                    as_str(): "my_array[0]",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 205,
                                                                            as_str(): "my_array[0]",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09298f300,
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
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 205,
                                                                            as_str(): "my_array[0]",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                548,
                                                                Span {
                                                                    src (ptr): 0x00007fe09298f300,
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
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 205,
                                                            as_str(): "my_array[0]",
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
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 225,
                                                                                            end: 226,
                                                                                            as_str(): "x",
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
                                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 194,
                                                                                                            end: 205,
                                                                                                            as_str(): "my_array[0]",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 194,
                                                                                                        end: 205,
                                                                                                        as_str(): "my_array[0]",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7256,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 194,
                                                                                                    end: 205,
                                                                                                    as_str(): "my_array[0]",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 53,
                                                                                                        end: 56,
                                                                                                        as_str(): "Bar",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    7253,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7252,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 58,
                                                                                                    end: 59,
                                                                                                    as_str(): "X",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 53,
                                                                                                    end: 59,
                                                                                                    as_str(): "Bar: X",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7253,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 216,
                                                                                            end: 227,
                                                                                            as_str(): "Foo::Bar(x)",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        7253,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        7253,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 225,
                                                                            end: 226,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: StructFieldAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 225,
                                                                                                    end: 226,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 231,
                                                                                                end: 232,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7253,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 231,
                                                                                            end: 232,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: TyStructField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 24,
                                                                                                end: 29,
                                                                                                as_str(): "value",
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
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 24,
                                                                                            end: 34,
                                                                                            as_str(): "value: u64",
                                                                                        },
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 31,
                                                                                            end: 34,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    field_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 238,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7253,
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 231,
                                                                                    end: 238,
                                                                                    as_str(): "x.value",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 231,
                                                                            end: 238,
                                                                            as_str(): "x.value",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 231,
                                                            end: 238,
                                                            as_str(): "x.value",
                                                        },
                                                    },
                                                    else: Some(
                                                        TyExpression {
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
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 225,
                                                                                                end: 226,
                                                                                                as_str(): "x",
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
                                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 194,
                                                                                                                end: 205,
                                                                                                                as_str(): "my_array[0]",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 194,
                                                                                                            end: 205,
                                                                                                            as_str(): "my_array[0]",
                                                                                                        },
                                                                                                        mutability: Immutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        7256,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 194,
                                                                                                        end: 205,
                                                                                                        as_str(): "my_array[0]",
                                                                                                    },
                                                                                                },
                                                                                                variant: TyEnumVariant {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 53,
                                                                                                            end: 56,
                                                                                                            as_str(): "Bar",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    type_id: TypeId(
                                                                                                        7253,
                                                                                                    ),
                                                                                                    initial_type_id: TypeId(
                                                                                                        7252,
                                                                                                    ),
                                                                                                    type_span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 58,
                                                                                                        end: 59,
                                                                                                        as_str(): "X",
                                                                                                    },
                                                                                                    tag: 0,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 53,
                                                                                                        end: 59,
                                                                                                        as_str(): "Bar: X",
                                                                                                    },
                                                                                                    attributes: {},
                                                                                                },
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                7253,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 216,
                                                                                                end: 227,
                                                                                                as_str(): "Foo::Bar(x)",
                                                                                            },
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                        return_type: TypeId(
                                                                                            7253,
                                                                                        ),
                                                                                        type_ascription: TypeId(
                                                                                            7253,
                                                                                        ),
                                                                                        type_ascription_span: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 225,
                                                                                end: 226,
                                                                                as_str(): "x",
                                                                            },
                                                                        },
                                                                        TyAstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                TyExpression {
                                                                                    expression: StructFieldAccess {
                                                                                        prefix: TyExpression {
                                                                                            expression: VariableExpression {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 225,
                                                                                                        end: 226,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 231,
                                                                                                    end: 232,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                7253,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 231,
                                                                                                end: 232,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: TyStructField {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 24,
                                                                                                    end: 29,
                                                                                                    as_str(): "value",
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
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 24,
                                                                                                end: 34,
                                                                                                as_str(): "value: u64",
                                                                                            },
                                                                                            type_span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 31,
                                                                                                end: 34,
                                                                                                as_str(): "u64",
                                                                                            },
                                                                                            attributes: {},
                                                                                        },
                                                                                        field_instantiation_span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 233,
                                                                                            end: 238,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        resolved_type_of_parent: TypeId(
                                                                                            7253,
                                                                                        ),
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 231,
                                                                                        end: 238,
                                                                                        as_str(): "x.value",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 231,
                                                                                end: 238,
                                                                                as_str(): "x.value",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 231,
                                                                end: 238,
                                                                as_str(): "x.value",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0935f24b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                    ),
                                                    start: 231,
                                                    end: 238,
                                                    as_str(): "x.value",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 188,
                                            end: 245,
                                            as_str(): "match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0935f24b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                            ),
                            start: 188,
                            end: 245,
                            as_str(): "match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0935f24b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                    ),
                    start: 188,
                    end: 245,
                    as_str(): "match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0935f24b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
        ),
        start: 64,
        end: 247,
        as_str(): "fn main() -> u64 {\n    let mut my_array: [Foo; 1] = [Foo::Bar(X{value: 10})];\n    my_array[0] = Foo::Bar(X{value: 20});\n    match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }\n}",
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
        src (ptr): 0x00007fe0935f24b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
        ),
        start: 77,
        end: 80,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

