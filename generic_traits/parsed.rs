[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 238,
                    end: 252,
                    as_str(): "dep my_double;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 242,
                    end: 251,
                    as_str(): "my_double",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 238,
            end: 252,
            as_str(): "dep my_double;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 253,
                    end: 266,
                    as_str(): "dep my_point;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 257,
                    end: 265,
                    as_str(): "my_point",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 253,
            end: 266,
            as_str(): "dep my_point;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 267,
                    end: 281,
                    as_str(): "dep my_triple;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 271,
                    end: 280,
                    as_str(): "my_triple",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 267,
            end: 281,
            as_str(): "dep my_triple;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 287,
                            end: 295,
                            as_str(): "my_point",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 297,
                            end: 304,
                            as_str(): "MyPoint",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 283,
            end: 305,
            as_str(): "use my_point::MyPoint;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 310,
                            end: 319,
                            as_str(): "my_triple",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 321,
                            end: 329,
                            as_str(): "MyTriple",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 306,
            end: 330,
            as_str(): "use my_triple::MyTriple;",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 338,
                            end: 344,
                            as_str(): "Setter",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [
                        T: TypeId(7252),
                    ],
                    attributes: {},
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 357,
                                    end: 360,
                                    as_str(): "set",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 361,
                                            end: 365,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 361,
                                        end: 365,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 367,
                                            end: 376,
                                            as_str(): "new_value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 378,
                                                end: 379,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 378,
                                        end: 379,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            return_type: SelfType,
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 384,
                                end: 388,
                                as_str(): "Self",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 332,
                        end: 391,
                        as_str(): "trait Setter<T> {\n    fn set(self, new_value: T) -> Self;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 332,
            end: 391,
            as_str(): "trait Setter<T> {\n    fn set(self, new_value: T) -> Self;\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 400,
                            end: 410,
                            as_str(): "FooBarData",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 420,
                                    end: 425,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 427,
                                        end: 428,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 420,
                                end: 428,
                                as_str(): "value: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 427,
                                end: 428,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7253),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 393,
                        end: 430,
                        as_str(): "struct FooBarData<T> {\n    value: T\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 393,
            end: 430,
            as_str(): "struct FooBarData<T> {\n    value: T\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [
                        T: TypeId(7255),
                    ],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 440,
                                end: 446,
                                as_str(): "Setter",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                7256,
                            ),
                            initial_type_id: TypeId(
                                7256,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 447,
                                end: 448,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 454,
                                end: 464,
                                as_str(): "FooBarData",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7254,
                                    ),
                                    initial_type_id: TypeId(
                                        7254,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 465,
                                        end: 466,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 454,
                        end: 467,
                        as_str(): "FooBarData<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 477,
                                    end: 480,
                                    as_str(): "set",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 519,
                                                                        end: 529,
                                                                        as_str(): "FooBarData",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 519,
                                                                end: 529,
                                                                as_str(): "FooBarData",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 544,
                                                                        end: 549,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 551,
                                                                                end: 560,
                                                                                as_str(): "new_value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 551,
                                                                        end: 560,
                                                                        as_str(): "new_value",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 544,
                                                                    end: 560,
                                                                    as_str(): "value: new_value",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 519,
                                                    end: 571,
                                                    as_str(): "FooBarData {\n            value: new_value,\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 519,
                                            end: 571,
                                            as_str(): "FooBarData {\n            value: new_value,\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 509,
                                    end: 577,
                                    as_str(): "{\n        FooBarData {\n            value: new_value,\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 481,
                                            end: 485,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 481,
                                        end: 485,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 487,
                                            end: 496,
                                            as_str(): "new_value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 498,
                                                end: 499,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 498,
                                        end: 499,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 474,
                                end: 577,
                                as_str(): "fn set(self, new_value: T) -> Self {\n        FooBarData {\n            value: new_value,\n        }\n    }",
                            },
                            return_type: SelfType,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 504,
                                end: 508,
                                as_str(): "Self",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 432,
                        end: 579,
                        as_str(): "impl<T> Setter<T> for FooBarData<T> {\n    fn set(self, new_value: T) -> Self {\n        FooBarData {\n            value: new_value,\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 432,
            end: 579,
            as_str(): "impl<T> Setter<T> for FooBarData<T> {\n    fn set(self, new_value: T) -> Self {\n        FooBarData {\n            value: new_value,\n        }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 587,
                            end: 595,
                            as_str(): "Returner",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [
                        T: TypeId(7257),
                    ],
                    attributes: {},
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 608,
                                    end: 617,
                                    as_str(): "return_it",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 618,
                                            end: 622,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 618,
                                        end: 622,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 624,
                                            end: 633,
                                            as_str(): "the_value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 635,
                                                end: 636,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 635,
                                        end: 636,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 641,
                                        end: 642,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 641,
                                end: 642,
                                as_str(): "T",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 581,
                        end: 645,
                        as_str(): "trait Returner<T> {\n    fn return_it(self, the_value: T) -> T;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 581,
            end: 645,
            as_str(): "trait Returner<T> {\n    fn return_it(self, the_value: T) -> T;\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [
                        T: TypeId(7259),
                        F: TypeId(7260),
                    ],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 658,
                                end: 666,
                                as_str(): "Returner",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                7261,
                            ),
                            initial_type_id: TypeId(
                                7261,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 667,
                                end: 668,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 674,
                                end: 684,
                                as_str(): "FooBarData",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7258,
                                    ),
                                    initial_type_id: TypeId(
                                        7258,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 685,
                                        end: 686,
                                        as_str(): "F",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 674,
                        end: 687,
                        as_str(): "FooBarData<F>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 697,
                                    end: 706,
                                    as_str(): "return_it",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 742,
                                                            end: 751,
                                                            as_str(): "the_value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 742,
                                                    end: 751,
                                                    as_str(): "the_value",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 742,
                                            end: 751,
                                            as_str(): "the_value",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 732,
                                    end: 757,
                                    as_str(): "{\n        the_value\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 707,
                                            end: 711,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 707,
                                        end: 711,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 713,
                                            end: 722,
                                            as_str(): "the_value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 724,
                                                end: 725,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 724,
                                        end: 725,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 694,
                                end: 757,
                                as_str(): "fn return_it(self, the_value: T) -> T {\n        the_value\n    }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 730,
                                        end: 731,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 730,
                                end: 731,
                                as_str(): "T",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 647,
                        end: 759,
                        as_str(): "impl<T, F> Returner<T> for FooBarData<F> {\n    fn return_it(self, the_value: T) -> T {\n        the_value\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 647,
            end: 759,
            as_str(): "impl<T, F> Returner<T> for FooBarData<F> {\n    fn return_it(self, the_value: T) -> T {\n        the_value\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 767,
                            end: 772,
                            as_str(): "MyAdd",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [
                        T: TypeId(7262),
                    ],
                    attributes: {},
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 785,
                                    end: 791,
                                    as_str(): "my_add",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 792,
                                            end: 796,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 792,
                                        end: 796,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 798,
                                            end: 799,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 801,
                                                end: 802,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 801,
                                        end: 802,
                                        as_str(): "T",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 804,
                                            end: 805,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 807,
                                                end: 808,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 807,
                                        end: 808,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 813,
                                        end: 814,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 813,
                                end: 814,
                                as_str(): "T",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 761,
                        end: 817,
                        as_str(): "trait MyAdd<T> {\n    fn my_add(self, a: T, b: T) -> T;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 761,
            end: 817,
            as_str(): "trait MyAdd<T> {\n    fn my_add(self, a: T, b: T) -> T;\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [
                        T: TypeId(7264),
                    ],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 945,
                                end: 950,
                                as_str(): "MyAdd",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 951,
                                end: 954,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 960,
                                end: 970,
                                as_str(): "FooBarData",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7263,
                                    ),
                                    initial_type_id: TypeId(
                                        7263,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 971,
                                        end: 972,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 960,
                        end: 973,
                        as_str(): "FooBarData<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 983,
                                    end: 989,
                                    as_str(): "my_add",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1031,
                                                                                end: 1032,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1031,
                                                                                end: 1032,
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
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1031,
                                                                            end: 1032,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 1031,
                                                                end: 1032,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1029,
                                                                            end: 1030,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1029,
                                                                    end: 1030,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1033,
                                                                            end: 1034,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1033,
                                                                    end: 1034,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1029,
                                                    end: 1034,
                                                    as_str(): "a + b",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1029,
                                            end: 1034,
                                            as_str(): "a + b",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1019,
                                    end: 1040,
                                    as_str(): "{\n        a + b\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 990,
                                            end: 994,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 990,
                                        end: 994,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 996,
                                            end: 997,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 999,
                                        end: 1002,
                                        as_str(): "u64",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1004,
                                            end: 1005,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1007,
                                        end: 1010,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 980,
                                end: 1040,
                                as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1015,
                                end: 1018,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 937,
                        end: 1042,
                        as_str(): "impl<T> MyAdd<u64> for FooBarData<T> {\n    fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 937,
            end: 1042,
            as_str(): "impl<T> MyAdd<u64> for FooBarData<T> {\n    fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 1050,
                            end: 1055,
                            as_str(): "MySub",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [
                        T: TypeId(7265),
                    ],
                    attributes: {},
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1068,
                                    end: 1074,
                                    as_str(): "my_sub",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1075,
                                            end: 1076,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 1078,
                                                end: 1079,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1078,
                                        end: 1079,
                                        as_str(): "T",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1081,
                                            end: 1082,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 1084,
                                                end: 1085,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1084,
                                        end: 1085,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1090,
                                        end: 1091,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1090,
                                end: 1091,
                                as_str(): "T",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1044,
                        end: 1094,
                        as_str(): "trait MySub<T> {\n    fn my_sub(a: T, b: T) -> T;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1044,
            end: 1094,
            as_str(): "trait MySub<T> {\n    fn my_sub(a: T, b: T) -> T;\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [
                        T: TypeId(7267),
                    ],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1297,
                                end: 1302,
                                as_str(): "MySub",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1303,
                                end: 1306,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1312,
                                end: 1322,
                                as_str(): "FooBarData",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7266,
                                    ),
                                    initial_type_id: TypeId(
                                        7266,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1323,
                                        end: 1324,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1312,
                        end: 1325,
                        as_str(): "FooBarData<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1335,
                                    end: 1341,
                                    as_str(): "my_sub",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
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
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 1380,
                                                                                            end: 1382,
                                                                                            as_str(): ">=",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 1380,
                                                                                            end: 1382,
                                                                                            as_str(): ">=",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ge",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 1380,
                                                                                        end: 1382,
                                                                                        as_str(): ">=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1380,
                                                                            end: 1382,
                                                                            as_str(): ">=",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 1378,
                                                                                        end: 1379,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1378,
                                                                                end: 1379,
                                                                                as_str(): "a",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 1383,
                                                                                        end: 1384,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1383,
                                                                                end: 1384,
                                                                                as_str(): "b",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 1378,
                                                                end: 1384,
                                                                as_str(): "a >= b",
                                                            },
                                                        },
                                                        then: Expression {
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
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1401,
                                                                                                                    end: 1402,
                                                                                                                    as_str(): "-",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1401,
                                                                                                                    end: 1402,
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
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1401,
                                                                                                                end: 1402,
                                                                                                                as_str(): "-",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: true,
                                                                                                    },
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1401,
                                                                                                    end: 1402,
                                                                                                    as_str(): "-",
                                                                                                },
                                                                                            },
                                                                                            contract_call_params: [],
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1399,
                                                                                                                end: 1400,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1399,
                                                                                                        end: 1400,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1403,
                                                                                                                end: 1404,
                                                                                                                as_str(): "b",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1403,
                                                                                                        end: 1404,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 1399,
                                                                                        end: 1404,
                                                                                        as_str(): "a - b",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1399,
                                                                                end: 1404,
                                                                                as_str(): "a - b",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 1385,
                                                                        end: 1414,
                                                                        as_str(): "{\n            a - b\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 1385,
                                                                end: 1414,
                                                                as_str(): "{\n            a - b\n        }",
                                                            },
                                                        },
                                                        else: Some(
                                                            Expression {
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1436,
                                                                                                                        end: 1437,
                                                                                                                        as_str(): "-",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1436,
                                                                                                                        end: 1437,
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
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1436,
                                                                                                                    end: 1437,
                                                                                                                    as_str(): "-",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1436,
                                                                                                        end: 1437,
                                                                                                        as_str(): "-",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1434,
                                                                                                                    end: 1435,
                                                                                                                    as_str(): "b",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1434,
                                                                                                            end: 1435,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1438,
                                                                                                                    end: 1439,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1438,
                                                                                                            end: 1439,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 1434,
                                                                                            end: 1439,
                                                                                            as_str(): "b - a",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 1434,
                                                                                    end: 1439,
                                                                                    as_str(): "b - a",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1420,
                                                                            end: 1449,
                                                                            as_str(): "{\n            b - a\n        }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1420,
                                                                    end: 1449,
                                                                    as_str(): "{\n            b - a\n        }",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1375,
                                                    end: 1449,
                                                    as_str(): "if a >= b {\n            a - b\n        } else {\n            b - a\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1375,
                                            end: 1449,
                                            as_str(): "if a >= b {\n            a - b\n        } else {\n            b - a\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1365,
                                    end: 1455,
                                    as_str(): "{\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1342,
                                            end: 1343,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1345,
                                        end: 1348,
                                        as_str(): "u64",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1350,
                                            end: 1351,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1353,
                                        end: 1356,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1332,
                                end: 1455,
                                as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1361,
                                end: 1364,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1289,
                        end: 1457,
                        as_str(): "impl<T> MySub<u64> for FooBarData<T> {\n    fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1289,
            end: 1457,
            as_str(): "impl<T> MySub<u64> for FooBarData<T> {\n    fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 1466,
                            end: 1475,
                            as_str(): "OtherData",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1485,
                                    end: 1486,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1488,
                                        end: 1489,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1485,
                                end: 1489,
                                as_str(): "a: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1488,
                                end: 1489,
                                as_str(): "T",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1495,
                                    end: 1496,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1498,
                                        end: 1499,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1495,
                                end: 1499,
                                as_str(): "b: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1498,
                                end: 1499,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7268),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1459,
                        end: 1502,
                        as_str(): "struct OtherData<T> {\n    a: T,\n    b: T,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1459,
            end: 1502,
            as_str(): "struct OtherData<T> {\n    a: T,\n    b: T,\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [
                        T: TypeId(7270),
                    ],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1629,
                                end: 1634,
                                as_str(): "MyAdd",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1635,
                                end: 1638,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1644,
                                end: 1653,
                                as_str(): "OtherData",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7269,
                                    ),
                                    initial_type_id: TypeId(
                                        7269,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1654,
                                        end: 1655,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1644,
                        end: 1656,
                        as_str(): "OtherData<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1666,
                                    end: 1672,
                                    as_str(): "my_add",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1714,
                                                                                end: 1715,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 1714,
                                                                                end: 1715,
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
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1714,
                                                                            end: 1715,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 1714,
                                                                end: 1715,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1712,
                                                                            end: 1713,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1712,
                                                                    end: 1713,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 1716,
                                                                            end: 1717,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1716,
                                                                    end: 1717,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1712,
                                                    end: 1717,
                                                    as_str(): "a + b",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1712,
                                            end: 1717,
                                            as_str(): "a + b",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1702,
                                    end: 1723,
                                    as_str(): "{\n        a + b\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1673,
                                            end: 1677,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1673,
                                        end: 1677,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1679,
                                            end: 1680,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1682,
                                        end: 1685,
                                        as_str(): "u64",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1687,
                                            end: 1688,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1690,
                                        end: 1693,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1663,
                                end: 1723,
                                as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1698,
                                end: 1701,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1621,
                        end: 1725,
                        as_str(): "impl<T> MyAdd<u64> for OtherData<T> {\n    fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1621,
            end: 1725,
            as_str(): "impl<T> MyAdd<u64> for OtherData<T> {\n    fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [
                        T: TypeId(7272),
                    ],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1927,
                                end: 1932,
                                as_str(): "MySub",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1933,
                                end: 1936,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1942,
                                end: 1951,
                                as_str(): "OtherData",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7271,
                                    ),
                                    initial_type_id: TypeId(
                                        7271,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1952,
                                        end: 1953,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1942,
                        end: 1954,
                        as_str(): "OtherData<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1964,
                                    end: 1970,
                                    as_str(): "my_sub",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
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
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2009,
                                                                                            end: 2011,
                                                                                            as_str(): ">=",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2009,
                                                                                            end: 2011,
                                                                                            as_str(): ">=",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ge",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2009,
                                                                                        end: 2011,
                                                                                        as_str(): ">=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2009,
                                                                            end: 2011,
                                                                            as_str(): ">=",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2007,
                                                                                        end: 2008,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2007,
                                                                                end: 2008,
                                                                                as_str(): "a",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2012,
                                                                                        end: 2013,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2012,
                                                                                end: 2013,
                                                                                as_str(): "b",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2007,
                                                                end: 2013,
                                                                as_str(): "a >= b",
                                                            },
                                                        },
                                                        then: Expression {
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
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2030,
                                                                                                                    end: 2031,
                                                                                                                    as_str(): "-",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2030,
                                                                                                                    end: 2031,
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
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2030,
                                                                                                                end: 2031,
                                                                                                                as_str(): "-",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: true,
                                                                                                    },
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2030,
                                                                                                    end: 2031,
                                                                                                    as_str(): "-",
                                                                                                },
                                                                                            },
                                                                                            contract_call_params: [],
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2028,
                                                                                                                end: 2029,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2028,
                                                                                                        end: 2029,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2032,
                                                                                                                end: 2033,
                                                                                                                as_str(): "b",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2032,
                                                                                                        end: 2033,
                                                                                                        as_str(): "b",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2028,
                                                                                        end: 2033,
                                                                                        as_str(): "a - b",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2028,
                                                                                end: 2033,
                                                                                as_str(): "a - b",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2014,
                                                                        end: 2043,
                                                                        as_str(): "{\n            a - b\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2014,
                                                                end: 2043,
                                                                as_str(): "{\n            a - b\n        }",
                                                            },
                                                        },
                                                        else: Some(
                                                            Expression {
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 2065,
                                                                                                                        end: 2066,
                                                                                                                        as_str(): "-",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 2065,
                                                                                                                        end: 2066,
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
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2065,
                                                                                                                    end: 2066,
                                                                                                                    as_str(): "-",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2065,
                                                                                                        end: 2066,
                                                                                                        as_str(): "-",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2063,
                                                                                                                    end: 2064,
                                                                                                                    as_str(): "b",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2063,
                                                                                                            end: 2064,
                                                                                                            as_str(): "b",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2067,
                                                                                                                    end: 2068,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2067,
                                                                                                            end: 2068,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2063,
                                                                                            end: 2068,
                                                                                            as_str(): "b - a",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2063,
                                                                                    end: 2068,
                                                                                    as_str(): "b - a",
                                                                                },
                                                                            },
                                                                        ],
                                                                        whole_block_span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2049,
                                                                            end: 2078,
                                                                            as_str(): "{\n            b - a\n        }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2049,
                                                                    end: 2078,
                                                                    as_str(): "{\n            b - a\n        }",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2004,
                                                    end: 2078,
                                                    as_str(): "if a >= b {\n            a - b\n        } else {\n            b - a\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2004,
                                            end: 2078,
                                            as_str(): "if a >= b {\n            a - b\n        } else {\n            b - a\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 1994,
                                    end: 2084,
                                    as_str(): "{\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1971,
                                            end: 1972,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1974,
                                        end: 1977,
                                        as_str(): "u64",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1979,
                                            end: 1980,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 1982,
                                        end: 1985,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1961,
                                end: 2084,
                                as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 1990,
                                end: 1993,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 1919,
                        end: 2086,
                        as_str(): "impl<T> MySub<u64> for OtherData<T> {\n    fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1919,
            end: 2086,
            as_str(): "impl<T> MySub<u64> for OtherData<T> {\n    fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2093,
                                end: 2101,
                                as_str(): "MyTriple",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2102,
                                end: 2105,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2111,
                                end: 2118,
                                as_str(): "MyPoint",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        21,
                                    ),
                                    initial_type_id: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2119,
                                        end: 2122,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2111,
                        end: 2123,
                        as_str(): "MyPoint<u64>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2133,
                                    end: 2142,
                                    as_str(): "my_triple",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2202,
                                                                                end: 2203,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2202,
                                                                                end: 2203,
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
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2202,
                                                                            end: 2203,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2202,
                                                                end: 2203,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2189,
                                                                                                end: 2190,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2189,
                                                                                                end: 2190,
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
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2189,
                                                                                            end: 2190,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2189,
                                                                                end: 2190,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
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
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2185,
                                                                                                                end: 2186,
                                                                                                                as_str(): "*",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2185,
                                                                                                                end: 2186,
                                                                                                                as_str(): "*",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "multiply",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2185,
                                                                                                            end: 2186,
                                                                                                            as_str(): "*",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2185,
                                                                                                end: 2186,
                                                                                                as_str(): "*",
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 2179,
                                                                                                                        end: 2183,
                                                                                                                        as_str(): "self",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2179,
                                                                                                                end: 2183,
                                                                                                                as_str(): "self",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2184,
                                                                                                                end: 2185,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2179,
                                                                                                    end: 2185,
                                                                                                    as_str(): "self.x",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2186,
                                                                                                    end: 2187,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2179,
                                                                                    end: 2187,
                                                                                    as_str(): "self.x*3",
                                                                                },
                                                                            },
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
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2198,
                                                                                                                end: 2199,
                                                                                                                as_str(): "*",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2198,
                                                                                                                end: 2199,
                                                                                                                as_str(): "*",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "multiply",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2198,
                                                                                                            end: 2199,
                                                                                                            as_str(): "*",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2198,
                                                                                                end: 2199,
                                                                                                as_str(): "*",
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 2192,
                                                                                                                        end: 2196,
                                                                                                                        as_str(): "self",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2192,
                                                                                                                end: 2196,
                                                                                                                as_str(): "self",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2197,
                                                                                                                end: 2198,
                                                                                                                as_str(): "y",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2192,
                                                                                                    end: 2198,
                                                                                                    as_str(): "self.y",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        3,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2199,
                                                                                                    end: 2200,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2192,
                                                                                    end: 2200,
                                                                                    as_str(): "self.y*3",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2178,
                                                                    end: 2201,
                                                                    as_str(): "(self.x*3) + (self.y*3)",
                                                                },
                                                            },
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2210,
                                                                                                end: 2211,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2210,
                                                                                                end: 2211,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "multiply",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2210,
                                                                                            end: 2211,
                                                                                            as_str(): "*",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2210,
                                                                                end: 2211,
                                                                                as_str(): "*",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2205,
                                                                                            end: 2210,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2205,
                                                                                    end: 2210,
                                                                                    as_str(): "value",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2211,
                                                                                    end: 2212,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2205,
                                                                    end: 2212,
                                                                    as_str(): "value*3",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2178,
                                                    end: 2213,
                                                    as_str(): "(self.x*3) + (self.y*3) + (value*3)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2178,
                                            end: 2213,
                                            as_str(): "(self.x*3) + (self.y*3) + (value*3)",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2168,
                                    end: 2219,
                                    as_str(): "{\n        (self.x*3) + (self.y*3) + (value*3)\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2143,
                                            end: 2147,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2143,
                                        end: 2147,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2149,
                                            end: 2154,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2156,
                                        end: 2159,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2130,
                                end: 2219,
                                as_str(): "fn my_triple(self, value: u64) -> u64 {\n        (self.x*3) + (self.y*3) + (value*3)\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2164,
                                end: 2167,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2088,
                        end: 2221,
                        as_str(): "impl MyTriple<u64> for MyPoint<u64> {\n    fn my_triple(self, value: u64) -> u64 {\n        (self.x*3) + (self.y*3) + (value*3)\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2088,
            end: 2221,
            as_str(): "impl MyTriple<u64> for MyPoint<u64> {\n    fn my_triple(self, value: u64) -> u64 {\n        (self.x*3) + (self.y*3) + (value*3)\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 2230,
                            end: 2235,
                            as_str(): "MyU64",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2242,
                                    end: 2247,
                                    as_str(): "inner",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2242,
                                end: 2252,
                                as_str(): "inner: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2249,
                                end: 2252,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2223,
                        end: 2254,
                        as_str(): "struct MyU64 {\n    inner: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2223,
            end: 2254,
            as_str(): "struct MyU64 {\n    inner: u64\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2261,
                                end: 2269,
                                as_str(): "MyTriple",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2270,
                                end: 2273,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2279,
                                end: 2284,
                                as_str(): "MyU64",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2279,
                        end: 2284,
                        as_str(): "MyU64",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2294,
                                    end: 2303,
                                    as_str(): "my_triple",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2354,
                                                                                end: 2355,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2354,
                                                                                end: 2355,
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
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2354,
                                                                            end: 2355,
                                                                            as_str(): "+",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2354,
                                                                end: 2355,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2350,
                                                                                                end: 2351,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2350,
                                                                                                end: 2351,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "multiply",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2350,
                                                                                            end: 2351,
                                                                                            as_str(): "*",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2350,
                                                                                end: 2351,
                                                                                as_str(): "*",
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
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2340,
                                                                                                        end: 2344,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2340,
                                                                                                end: 2344,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        field_to_access: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2345,
                                                                                                end: 2350,
                                                                                                as_str(): "inner",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2340,
                                                                                    end: 2350,
                                                                                    as_str(): "self.inner",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2351,
                                                                                    end: 2352,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2340,
                                                                    end: 2352,
                                                                    as_str(): "self.inner*3",
                                                                },
                                                            },
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2362,
                                                                                                end: 2363,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2362,
                                                                                                end: 2363,
                                                                                                as_str(): "*",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "multiply",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2362,
                                                                                            end: 2363,
                                                                                            as_str(): "*",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2362,
                                                                                end: 2363,
                                                                                as_str(): "*",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2357,
                                                                                            end: 2362,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2357,
                                                                                    end: 2362,
                                                                                    as_str(): "value",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2363,
                                                                                    end: 2364,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2357,
                                                                    end: 2364,
                                                                    as_str(): "value*3",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2339,
                                                    end: 2365,
                                                    as_str(): "(self.inner*3) + (value*3)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2339,
                                            end: 2365,
                                            as_str(): "(self.inner*3) + (value*3)",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2329,
                                    end: 2371,
                                    as_str(): "{\n        (self.inner*3) + (value*3)\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2304,
                                            end: 2308,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2304,
                                        end: 2308,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2310,
                                            end: 2315,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2317,
                                        end: 2320,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2291,
                                end: 2371,
                                as_str(): "fn my_triple(self, value: u64) -> u64 {\n        (self.inner*3) + (value*3)\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fd90ad70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                ),
                                start: 2325,
                                end: 2328,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2256,
                        end: 2373,
                        as_str(): "impl MyTriple<u64> for MyU64 {\n    fn my_triple(self, value: u64) -> u64 {\n        (self.inner*3) + (value*3)\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2256,
            end: 2373,
            as_str(): "impl MyTriple<u64> for MyU64 {\n    fn my_triple(self, value: u64) -> u64 {\n        (self.inner*3) + (value*3)\n    }\n}",
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
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 2378,
                            end: 2382,
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2402,
                                                    end: 2403,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2406,
                                                                        end: 2416,
                                                                        as_str(): "FooBarData",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2406,
                                                                end: 2416,
                                                                as_str(): "FooBarData",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2427,
                                                                        end: 2432,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U8(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2434,
                                                                        end: 2437,
                                                                        as_str(): "1u8",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2427,
                                                                    end: 2437,
                                                                    as_str(): "value: 1u8",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2406,
                                                    end: 2443,
                                                    as_str(): "FooBarData {\n        value: 1u8\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2398,
                                    end: 2444,
                                    as_str(): "let a = FooBarData {\n        value: 1u8\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2453,
                                                    end: 2454,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2459,
                                                                        end: 2462,
                                                                        as_str(): "set",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2459,
                                                                end: 2462,
                                                                as_str(): "set",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2457,
                                                                            end: 2458,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2457,
                                                                    end: 2458,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        42,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2463,
                                                                    end: 2465,
                                                                    as_str(): "42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2457,
                                                    end: 2466,
                                                    as_str(): "a.set(42)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2449,
                                    end: 2467,
                                    as_str(): "let b = a.set(42);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2476,
                                                    end: 2477,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2480,
                                                                        end: 2481,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2480,
                                                                end: 2481,
                                                                as_str(): "b",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2482,
                                                                end: 2487,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2480,
                                                    end: 2487,
                                                    as_str(): "b.value",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2472,
                                    end: 2488,
                                    as_str(): "let c = b.value;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2497,
                                                    end: 2498,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2503,
                                                                        end: 2512,
                                                                        as_str(): "return_it",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2503,
                                                                end: 2512,
                                                                as_str(): "return_it",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2501,
                                                                            end: 2502,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2501,
                                                                    end: 2502,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2513,
                                                                    end: 2517,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2501,
                                                    end: 2518,
                                                    as_str(): "b.return_it(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2493,
                                    end: 2519,
                                    as_str(): "let d = b.return_it(true);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2528,
                                                    end: 2529,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2534,
                                                                        end: 2543,
                                                                        as_str(): "return_it",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2534,
                                                                end: 2543,
                                                                as_str(): "return_it",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2532,
                                                                            end: 2533,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2532,
                                                                    end: 2533,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        9,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2544,
                                                                    end: 2548,
                                                                    as_str(): "9u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2532,
                                                    end: 2549,
                                                    as_str(): "b.return_it(9u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2524,
                                    end: 2550,
                                    as_str(): "let e = b.return_it(9u64);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2559,
                                                    end: 2560,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2563,
                                                                        end: 2573,
                                                                        as_str(): "FooBarData",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2563,
                                                                end: 2573,
                                                                as_str(): "FooBarData",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2584,
                                                                        end: 2589,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2591,
                                                                        end: 2595,
                                                                        as_str(): "1u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2584,
                                                                    end: 2595,
                                                                    as_str(): "value: 1u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2563,
                                                    end: 2601,
                                                    as_str(): "FooBarData {\n        value: 1u64\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2555,
                                    end: 2602,
                                    as_str(): "let f = FooBarData {\n        value: 1u64\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2611,
                                                    end: 2612,
                                                    as_str(): "g",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2617,
                                                                        end: 2623,
                                                                        as_str(): "my_add",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2617,
                                                                end: 2623,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2615,
                                                                            end: 2616,
                                                                            as_str(): "f",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2615,
                                                                    end: 2616,
                                                                    as_str(): "f",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: MethodApplication(
                                                                    MethodApplicationExpression {
                                                                        method_name_binding: TypeBinding {
                                                                            inner: FromModule {
                                                                                method_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2626,
                                                                                        end: 2632,
                                                                                        as_str(): "my_add",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2626,
                                                                                end: 2632,
                                                                                as_str(): "my_add",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2624,
                                                                                            end: 2625,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2624,
                                                                                    end: 2625,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2633,
                                                                                    end: 2636,
                                                                                    as_str(): "1u8",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        2,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2638,
                                                                                    end: 2641,
                                                                                    as_str(): "2u8",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2624,
                                                                    end: 2642,
                                                                    as_str(): "a.my_add(1u8, 2u8)",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: MethodApplication(
                                                                    MethodApplicationExpression {
                                                                        method_name_binding: TypeBinding {
                                                                            inner: FromModule {
                                                                                method_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2646,
                                                                                        end: 2652,
                                                                                        as_str(): "my_add",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2646,
                                                                                end: 2652,
                                                                                as_str(): "my_add",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2644,
                                                                                            end: 2645,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2644,
                                                                                    end: 2645,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2653,
                                                                                    end: 2656,
                                                                                    as_str(): "3u8",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        4,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2658,
                                                                                    end: 2661,
                                                                                    as_str(): "4u8",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2644,
                                                                    end: 2662,
                                                                    as_str(): "a.my_add(3u8, 4u8)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2615,
                                                    end: 2663,
                                                    as_str(): "f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8))",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2607,
                                    end: 2664,
                                    as_str(): "let g = f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8));",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2673,
                                                    end: 2674,
                                                    as_str(): "h",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2677,
                                                                                end: 2687,
                                                                                as_str(): "FooBarData",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2690,
                                                                                    end: 2693,
                                                                                    as_str(): "u64",
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2677,
                                                                            end: 2694,
                                                                            as_str(): "FooBarData::<u64>",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2696,
                                                                            end: 2702,
                                                                            as_str(): "my_sub",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2677,
                                                                end: 2702,
                                                                as_str(): "FooBarData::<u64>::my_sub",
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2712,
                                                                                                end: 2722,
                                                                                                as_str(): "FooBarData",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [
                                                                                            TypeArgument {
                                                                                                type_id: TypeId(
                                                                                                    50,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    50,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2725,
                                                                                                    end: 2727,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2712,
                                                                                            end: 2728,
                                                                                            as_str(): "FooBarData::<u8>",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2730,
                                                                                            end: 2736,
                                                                                            as_str(): "my_sub",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2712,
                                                                                end: 2736,
                                                                                as_str(): "FooBarData::<u8>::my_sub",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        100,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2737,
                                                                                    end: 2740,
                                                                                    as_str(): "100",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        10,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2742,
                                                                                    end: 2744,
                                                                                    as_str(): "10",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2712,
                                                                    end: 2745,
                                                                    as_str(): "FooBarData::<u8>::my_sub(100, 10)",
                                                                },
                                                            },
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 2755,
                                                                                                end: 2765,
                                                                                                as_str(): "FooBarData",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [
                                                                                            TypeArgument {
                                                                                                type_id: TypeId(
                                                                                                    50,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    50,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2768,
                                                                                                    end: 2770,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2755,
                                                                                            end: 2771,
                                                                                            as_str(): "FooBarData::<u8>",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2773,
                                                                                            end: 2779,
                                                                                            as_str(): "my_sub",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2755,
                                                                                end: 2779,
                                                                                as_str(): "FooBarData::<u8>::my_sub",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        50,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2780,
                                                                                    end: 2782,
                                                                                    as_str(): "50",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        10,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2784,
                                                                                    end: 2786,
                                                                                    as_str(): "10",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2755,
                                                                    end: 2787,
                                                                    as_str(): "FooBarData::<u8>::my_sub(50, 10)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2677,
                                                    end: 2794,
                                                    as_str(): "FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2669,
                                    end: 2795,
                                    as_str(): "let h = FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    );",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2804,
                                                    end: 2805,
                                                    as_str(): "i",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2808,
                                                                        end: 2817,
                                                                        as_str(): "OtherData",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2808,
                                                                end: 2817,
                                                                as_str(): "OtherData",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2828,
                                                                        end: 2829,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2831,
                                                                        end: 2835,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2828,
                                                                    end: 2835,
                                                                    as_str(): "a: true",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2845,
                                                                        end: 2846,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Boolean(
                                                                            false,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2848,
                                                                        end: 2853,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2845,
                                                                    end: 2853,
                                                                    as_str(): "b: false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2808,
                                                    end: 2860,
                                                    as_str(): "OtherData {\n        a: true,\n        b: false,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2800,
                                    end: 2861,
                                    as_str(): "let i = OtherData {\n        a: true,\n        b: false,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2870,
                                                    end: 2871,
                                                    as_str(): "j",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2874,
                                                                        end: 2883,
                                                                        as_str(): "OtherData",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2874,
                                                                end: 2883,
                                                                as_str(): "OtherData",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2894,
                                                                        end: 2895,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2897,
                                                                        end: 2902,
                                                                        as_str(): "10u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2894,
                                                                    end: 2902,
                                                                    as_str(): "a: 10u32",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2912,
                                                                        end: 2913,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2915,
                                                                        end: 2920,
                                                                        as_str(): "11u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2912,
                                                                    end: 2920,
                                                                    as_str(): "b: 11u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2874,
                                                    end: 2927,
                                                    as_str(): "OtherData {\n        a: 10u32,\n        b: 11u32,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2866,
                                    end: 2928,
                                    as_str(): "let j = OtherData {\n        a: 10u32,\n        b: 11u32,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2937,
                                                    end: 2938,
                                                    as_str(): "k",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2943,
                                                                        end: 2949,
                                                                        as_str(): "my_add",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2943,
                                                                end: 2949,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2941,
                                                                            end: 2942,
                                                                            as_str(): "j",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2941,
                                                                    end: 2942,
                                                                    as_str(): "j",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: MethodApplication(
                                                                    MethodApplicationExpression {
                                                                        method_name_binding: TypeBinding {
                                                                            inner: FromModule {
                                                                                method_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2952,
                                                                                        end: 2958,
                                                                                        as_str(): "my_add",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2952,
                                                                                end: 2958,
                                                                                as_str(): "my_add",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2950,
                                                                                            end: 2951,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2950,
                                                                                    end: 2951,
                                                                                    as_str(): "i",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2959,
                                                                                    end: 2962,
                                                                                    as_str(): "1u8",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        2,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2964,
                                                                                    end: 2967,
                                                                                    as_str(): "2u8",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2950,
                                                                    end: 2968,
                                                                    as_str(): "i.my_add(1u8, 2u8)",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: MethodApplication(
                                                                    MethodApplicationExpression {
                                                                        method_name_binding: TypeBinding {
                                                                            inner: FromModule {
                                                                                method_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 2972,
                                                                                        end: 2978,
                                                                                        as_str(): "my_add",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 2972,
                                                                                end: 2978,
                                                                                as_str(): "my_add",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2970,
                                                                                            end: 2971,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2970,
                                                                                    end: 2971,
                                                                                    as_str(): "i",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2979,
                                                                                    end: 2982,
                                                                                    as_str(): "3u8",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U8(
                                                                                        4,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 2984,
                                                                                    end: 2987,
                                                                                    as_str(): "4u8",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2970,
                                                                    end: 2988,
                                                                    as_str(): "i.my_add(3u8, 4u8)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2941,
                                                    end: 2989,
                                                    as_str(): "j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8))",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2933,
                                    end: 2990,
                                    as_str(): "let k = j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8));",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2999,
                                                    end: 3000,
                                                    as_str(): "l",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3003,
                                                                                end: 3013,
                                                                                as_str(): "FooBarData",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    42,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    42,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3016,
                                                                                    end: 3019,
                                                                                    as_str(): "u16",
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3003,
                                                                            end: 3020,
                                                                            as_str(): "FooBarData::<u16>",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3022,
                                                                            end: 3028,
                                                                            as_str(): "my_sub",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3003,
                                                                end: 3028,
                                                                as_str(): "FooBarData::<u16>::my_sub",
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3038,
                                                                                                end: 3048,
                                                                                                as_str(): "FooBarData",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [
                                                                                            TypeArgument {
                                                                                                type_id: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3051,
                                                                                                    end: 3054,
                                                                                                    as_str(): "u32",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3038,
                                                                                            end: 3055,
                                                                                            as_str(): "FooBarData::<u32>",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3057,
                                                                                            end: 3063,
                                                                                            as_str(): "my_sub",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3038,
                                                                                end: 3063,
                                                                                as_str(): "FooBarData::<u32>::my_sub",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        100,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3064,
                                                                                    end: 3067,
                                                                                    as_str(): "100",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        10,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3069,
                                                                                    end: 3071,
                                                                                    as_str(): "10",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3038,
                                                                    end: 3072,
                                                                    as_str(): "FooBarData::<u32>::my_sub(100, 10)",
                                                                },
                                                            },
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3082,
                                                                                                end: 3092,
                                                                                                as_str(): "FooBarData",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [
                                                                                            TypeArgument {
                                                                                                type_id: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3095,
                                                                                                    end: 3098,
                                                                                                    as_str(): "u32",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3082,
                                                                                            end: 3099,
                                                                                            as_str(): "FooBarData::<u32>",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3101,
                                                                                            end: 3107,
                                                                                            as_str(): "my_sub",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3082,
                                                                                end: 3107,
                                                                                as_str(): "FooBarData::<u32>::my_sub",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        50,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3108,
                                                                                    end: 3110,
                                                                                    as_str(): "50",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        10,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3112,
                                                                                    end: 3114,
                                                                                    as_str(): "10",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3082,
                                                                    end: 3115,
                                                                    as_str(): "FooBarData::<u32>::my_sub(50, 10)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3003,
                                                    end: 3122,
                                                    as_str(): "FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2995,
                                    end: 3123,
                                    as_str(): "let l = FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    );",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3132,
                                                    end: 3133,
                                                    as_str(): "m",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3136,
                                                                        end: 3143,
                                                                        as_str(): "MyPoint",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3136,
                                                                end: 3143,
                                                                as_str(): "MyPoint",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3154,
                                                                        end: 3155,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3157,
                                                                        end: 3162,
                                                                        as_str(): "10u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3154,
                                                                    end: 3162,
                                                                    as_str(): "x: 10u64",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3172,
                                                                        end: 3173,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3175,
                                                                        end: 3180,
                                                                        as_str(): "10u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3172,
                                                                    end: 3180,
                                                                    as_str(): "y: 10u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3136,
                                                    end: 3187,
                                                    as_str(): "MyPoint {\n        x: 10u64,\n        y: 10u64,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3128,
                                    end: 3188,
                                    as_str(): "let m = MyPoint {\n        x: 10u64,\n        y: 10u64,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3197,
                                                    end: 3198,
                                                    as_str(): "n",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3203,
                                                                        end: 3212,
                                                                        as_str(): "my_double",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3203,
                                                                end: 3212,
                                                                as_str(): "my_double",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3201,
                                                                            end: 3202,
                                                                            as_str(): "m",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3201,
                                                                    end: 3202,
                                                                    as_str(): "m",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        100,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3213,
                                                                    end: 3216,
                                                                    as_str(): "100",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3201,
                                                    end: 3217,
                                                    as_str(): "m.my_double(100)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3193,
                                    end: 3218,
                                    as_str(): "let n = m.my_double(100);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3227,
                                                    end: 3228,
                                                    as_str(): "o",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3233,
                                                                        end: 3242,
                                                                        as_str(): "my_triple",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3233,
                                                                end: 3242,
                                                                as_str(): "my_triple",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3231,
                                                                            end: 3232,
                                                                            as_str(): "m",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3231,
                                                                    end: 3232,
                                                                    as_str(): "m",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        100,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3243,
                                                                    end: 3246,
                                                                    as_str(): "100",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3231,
                                                    end: 3247,
                                                    as_str(): "m.my_triple(100)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3223,
                                    end: 3248,
                                    as_str(): "let o = m.my_triple(100);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3257,
                                                    end: 3258,
                                                    as_str(): "p",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3261,
                                                                        end: 3266,
                                                                        as_str(): "MyU64",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3261,
                                                                end: 3266,
                                                                as_str(): "MyU64",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3277,
                                                                        end: 3282,
                                                                        as_str(): "inner",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            30,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3284,
                                                                        end: 3289,
                                                                        as_str(): "30u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3277,
                                                                    end: 3289,
                                                                    as_str(): "inner: 30u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3261,
                                                    end: 3295,
                                                    as_str(): "MyU64 {\n        inner: 30u64\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3253,
                                    end: 3296,
                                    as_str(): "let p = MyU64 {\n        inner: 30u64\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3305,
                                                    end: 3306,
                                                    as_str(): "q",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3311,
                                                                        end: 3320,
                                                                        as_str(): "my_triple",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3311,
                                                                end: 3320,
                                                                as_str(): "my_triple",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3309,
                                                                            end: 3310,
                                                                            as_str(): "p",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3309,
                                                                    end: 3310,
                                                                    as_str(): "p",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3321,
                                                                    end: 3322,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3309,
                                                    end: 3323,
                                                    as_str(): "p.my_triple(1)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3301,
                                    end: 3324,
                                    as_str(): "let q = p.my_triple(1);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: LazyOperator(
                                                        LazyOperatorExpression {
                                                            op: And,
                                                            lhs: Expression {
                                                                kind: LazyOperator(
                                                                    LazyOperatorExpression {
                                                                        op: And,
                                                                        lhs: Expression {
                                                                            kind: LazyOperator(
                                                                                LazyOperatorExpression {
                                                                                    op: And,
                                                                                    lhs: Expression {
                                                                                        kind: LazyOperator(
                                                                                            LazyOperatorExpression {
                                                                                                op: And,
                                                                                                lhs: Expression {
                                                                                                    kind: LazyOperator(
                                                                                                        LazyOperatorExpression {
                                                                                                            op: And,
                                                                                                            lhs: Expression {
                                                                                                                kind: LazyOperator(
                                                                                                                    LazyOperatorExpression {
                                                                                                                        op: And,
                                                                                                                        lhs: Expression {
                                                                                                                            kind: LazyOperator(
                                                                                                                                LazyOperatorExpression {
                                                                                                                                    op: And,
                                                                                                                                    lhs: Expression {
                                                                                                                                        kind: LazyOperator(
                                                                                                                                            LazyOperatorExpression {
                                                                                                                                                op: And,
                                                                                                                                                lhs: Expression {
                                                                                                                                                    kind: LazyOperator(
                                                                                                                                                        LazyOperatorExpression {
                                                                                                                                                            op: And,
                                                                                                                                                            lhs: Expression {
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
                                                                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                                ),
                                                                                                                                                                                                start: 3335,
                                                                                                                                                                                                end: 3337,
                                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                                            },
                                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                                        },
                                                                                                                                                                                        BaseIdent {
                                                                                                                                                                                            name_override_opt: Some(
                                                                                                                                                                                                "ops",
                                                                                                                                                                                            ),
                                                                                                                                                                                            span: Span {
                                                                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                                ),
                                                                                                                                                                                                start: 3335,
                                                                                                                                                                                                end: 3337,
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
                                                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 3335,
                                                                                                                                                                                            end: 3337,
                                                                                                                                                                                            as_str(): "==",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                    is_absolute: true,
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            type_arguments: [],
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 3335,
                                                                                                                                                                                end: 3337,
                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                        contract_call_params: [],
                                                                                                                                                                        arguments: [
                                                                                                                                                                            Expression {
                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 3333,
                                                                                                                                                                                            end: 3334,
                                                                                                                                                                                            as_str(): "c",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 3333,
                                                                                                                                                                                    end: 3334,
                                                                                                                                                                                    as_str(): "c",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            Expression {
                                                                                                                                                                                kind: Literal(
                                                                                                                                                                                    U8(
                                                                                                                                                                                        42,
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 3338,
                                                                                                                                                                                    end: 3342,
                                                                                                                                                                                    as_str(): "42u8",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        ],
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 3333,
                                                                                                                                                                    end: 3342,
                                                                                                                                                                    as_str(): "c == 42u8",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            rhs: Expression {
                                                                                                                                                                kind: Variable(
                                                                                                                                                                    BaseIdent {
                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 3354,
                                                                                                                                                                            end: 3355,
                                                                                                                                                                            as_str(): "d",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 3354,
                                                                                                                                                                    end: 3355,
                                                                                                                                                                    as_str(): "d",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 3333,
                                                                                                                                                        end: 3355,
                                                                                                                                                        as_str(): "c == 42u8\n        && d",
                                                                                                                                                    },
                                                                                                                                                },
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
                                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 3369,
                                                                                                                                                                                    end: 3371,
                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                },
                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                            },
                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                                    "ops",
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 3369,
                                                                                                                                                                                    end: 3371,
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
                                                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 3369,
                                                                                                                                                                                end: 3371,
                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: true,
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 3369,
                                                                                                                                                                    end: 3371,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            contract_call_params: [],
                                                                                                                                                            arguments: [
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: Variable(
                                                                                                                                                                        BaseIdent {
                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 3367,
                                                                                                                                                                                end: 3368,
                                                                                                                                                                                as_str(): "e",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 3367,
                                                                                                                                                                        end: 3368,
                                                                                                                                                                        as_str(): "e",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: Literal(
                                                                                                                                                                        U64(
                                                                                                                                                                            9,
                                                                                                                                                                        ),
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 3372,
                                                                                                                                                                        end: 3376,
                                                                                                                                                                        as_str(): "9u64",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 3367,
                                                                                                                                                        end: 3376,
                                                                                                                                                        as_str(): "e == 9u64",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 3333,
                                                                                                                                            end: 3376,
                                                                                                                                            as_str(): "c == 42u8\n        && d\n        && e == 9u64",
                                                                                                                                        },
                                                                                                                                    },
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
                                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 3390,
                                                                                                                                                                        end: 3392,
                                                                                                                                                                        as_str(): "==",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "ops",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 3390,
                                                                                                                                                                        end: 3392,
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
                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 3390,
                                                                                                                                                                    end: 3392,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            is_absolute: true,
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    type_arguments: [],
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 3390,
                                                                                                                                                        end: 3392,
                                                                                                                                                        as_str(): "==",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                contract_call_params: [],
                                                                                                                                                arguments: [
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Variable(
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 3388,
                                                                                                                                                                    end: 3389,
                                                                                                                                                                    as_str(): "g",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 3388,
                                                                                                                                                            end: 3389,
                                                                                                                                                            as_str(): "g",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Expression {
                                                                                                                                                        kind: Literal(
                                                                                                                                                            Numeric(
                                                                                                                                                                10,
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 3393,
                                                                                                                                                            end: 3395,
                                                                                                                                                            as_str(): "10",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 3388,
                                                                                                                                            end: 3395,
                                                                                                                                            as_str(): "g == 10",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 3333,
                                                                                                                                end: 3395,
                                                                                                                                as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10",
                                                                                                                            },
                                                                                                                        },
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
                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 3409,
                                                                                                                                                            end: 3411,
                                                                                                                                                            as_str(): "==",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: Some(
                                                                                                                                                            "ops",
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 3409,
                                                                                                                                                            end: 3411,
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
                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 3409,
                                                                                                                                                        end: 3411,
                                                                                                                                                        as_str(): "==",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                is_absolute: true,
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        type_arguments: [],
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 3409,
                                                                                                                                            end: 3411,
                                                                                                                                            as_str(): "==",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    contract_call_params: [],
                                                                                                                                    arguments: [
                                                                                                                                        Expression {
                                                                                                                                            kind: Variable(
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 3407,
                                                                                                                                                        end: 3408,
                                                                                                                                                        as_str(): "h",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 3407,
                                                                                                                                                end: 3408,
                                                                                                                                                as_str(): "h",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        Expression {
                                                                                                                                            kind: Literal(
                                                                                                                                                Numeric(
                                                                                                                                                    50,
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 3412,
                                                                                                                                                end: 3414,
                                                                                                                                                as_str(): "50",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ],
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 3407,
                                                                                                                                end: 3414,
                                                                                                                                as_str(): "h == 50",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3333,
                                                                                                                    end: 3414,
                                                                                                                    as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50",
                                                                                                                },
                                                                                                            },
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
                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 3428,
                                                                                                                                                end: 3430,
                                                                                                                                                as_str(): "==",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        BaseIdent {
                                                                                                                                            name_override_opt: Some(
                                                                                                                                                "ops",
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 3428,
                                                                                                                                                end: 3430,
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
                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 3428,
                                                                                                                                            end: 3430,
                                                                                                                                            as_str(): "==",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    is_absolute: true,
                                                                                                                                },
                                                                                                                            },
                                                                                                                            type_arguments: [],
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 3428,
                                                                                                                                end: 3430,
                                                                                                                                as_str(): "==",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        contract_call_params: [],
                                                                                                                        arguments: [
                                                                                                                            Expression {
                                                                                                                                kind: Variable(
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 3426,
                                                                                                                                            end: 3427,
                                                                                                                                            as_str(): "k",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 3426,
                                                                                                                                    end: 3427,
                                                                                                                                    as_str(): "k",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            Expression {
                                                                                                                                kind: Literal(
                                                                                                                                    Numeric(
                                                                                                                                        10,
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 3431,
                                                                                                                                    end: 3433,
                                                                                                                                    as_str(): "10",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ],
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3426,
                                                                                                                    end: 3433,
                                                                                                                    as_str(): "k == 10",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3333,
                                                                                                        end: 3433,
                                                                                                        as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10",
                                                                                                    },
                                                                                                },
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
                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 3447,
                                                                                                                                    end: 3449,
                                                                                                                                    as_str(): "==",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "ops",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 3447,
                                                                                                                                    end: 3449,
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
                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 3447,
                                                                                                                                end: 3449,
                                                                                                                                as_str(): "==",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        is_absolute: true,
                                                                                                                    },
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3447,
                                                                                                                    end: 3449,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                            },
                                                                                                            contract_call_params: [],
                                                                                                            arguments: [
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 3445,
                                                                                                                                end: 3446,
                                                                                                                                as_str(): "l",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3445,
                                                                                                                        end: 3446,
                                                                                                                        as_str(): "l",
                                                                                                                    },
                                                                                                                },
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            50,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3450,
                                                                                                                        end: 3452,
                                                                                                                        as_str(): "50",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3445,
                                                                                                        end: 3452,
                                                                                                        as_str(): "l == 50",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3333,
                                                                                            end: 3452,
                                                                                            as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50",
                                                                                        },
                                                                                    },
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3466,
                                                                                                                        end: 3468,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3466,
                                                                                                                        end: 3468,
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
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3466,
                                                                                                                    end: 3468,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3466,
                                                                                                        end: 3468,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3464,
                                                                                                                    end: 3465,
                                                                                                                    as_str(): "n",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3464,
                                                                                                            end: 3465,
                                                                                                            as_str(): "n",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                240,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3469,
                                                                                                            end: 3472,
                                                                                                            as_str(): "240",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3464,
                                                                                            end: 3472,
                                                                                            as_str(): "n == 240",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3333,
                                                                                end: 3472,
                                                                                as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240",
                                                                            },
                                                                        },
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
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3486,
                                                                                                            end: 3488,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3486,
                                                                                                            end: 3488,
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
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3486,
                                                                                                        end: 3488,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: true,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3486,
                                                                                            end: 3488,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3484,
                                                                                                        end: 3485,
                                                                                                        as_str(): "o",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3484,
                                                                                                end: 3485,
                                                                                                as_str(): "o",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    360,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3489,
                                                                                                end: 3492,
                                                                                                as_str(): "360",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3484,
                                                                                end: 3492,
                                                                                as_str(): "o == 360",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3333,
                                                                    end: 3492,
                                                                    as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360",
                                                                },
                                                            },
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3506,
                                                                                                end: 3508,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3506,
                                                                                                end: 3508,
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
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3506,
                                                                                            end: 3508,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3506,
                                                                                end: 3508,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3504,
                                                                                            end: 3505,
                                                                                            as_str(): "q",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3504,
                                                                                    end: 3505,
                                                                                    as_str(): "q",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        93,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3509,
                                                                                    end: 3511,
                                                                                    as_str(): "93",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3504,
                                                                    end: 3511,
                                                                    as_str(): "q == 93",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3333,
                                                        end: 3511,
                                                        as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: ImplicitReturnExpression(
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    42,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3522,
                                                                                end: 3524,
                                                                                as_str(): "42",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3522,
                                                                        end: 3524,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3512,
                                                                end: 3530,
                                                                as_str(): "{\n        42\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3512,
                                                        end: 3530,
                                                        as_str(): "{\n        42\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3546,
                                                                                    end: 3547,
                                                                                    as_str(): "7",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3546,
                                                                            end: 3547,
                                                                            as_str(): "7",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3536,
                                                                    end: 3553,
                                                                    as_str(): "{\n        7\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3536,
                                                            end: 3553,
                                                            as_str(): "{\n        7\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 3330,
                                            end: 3553,
                                            as_str(): "if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3330,
                                    end: 3553,
                                    as_str(): "if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 2392,
                            end: 3555,
                            as_str(): "{\n    let a = FooBarData {\n        value: 1u8\n    };\n    let b = a.set(42);\n    let c = b.value;\n    let d = b.return_it(true);\n    let e = b.return_it(9u64);\n    let f = FooBarData {\n        value: 1u64\n    };\n    let g = f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8));\n    let h = FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    );\n    let i = OtherData {\n        a: true,\n        b: false,\n    };\n    let j = OtherData {\n        a: 10u32,\n        b: 11u32,\n    };\n    let k = j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8));\n    let l = FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    );\n    let m = MyPoint {\n        x: 10u64,\n        y: 10u64,\n    };\n    let n = m.my_double(100);\n    let o = m.my_triple(100);\n    let p = MyU64 {\n        inner: 30u64\n    };\n    let q = p.my_triple(1);\n\n    if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2375,
                        end: 3555,
                        as_str(): "fn main() -> u64 {\n    let a = FooBarData {\n        value: 1u8\n    };\n    let b = a.set(42);\n    let c = b.value;\n    let d = b.return_it(true);\n    let e = b.return_it(9u64);\n    let f = FooBarData {\n        value: 1u64\n    };\n    let g = f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8));\n    let h = FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    );\n    let i = OtherData {\n        a: true,\n        b: false,\n    };\n    let j = OtherData {\n        a: 10u32,\n        b: 11u32,\n    };\n    let k = j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8));\n    let l = FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    );\n    let m = MyPoint {\n        x: 10u64,\n        y: 10u64,\n    };\n    let n = m.my_double(100);\n    let o = m.my_triple(100);\n    let p = MyU64 {\n        inner: 30u64\n    };\n    let q = p.my_triple(1);\n\n    if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fd90ad70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                        ),
                        start: 2388,
                        end: 2391,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2375,
            end: 3555,
            as_str(): "fn main() -> u64 {\n    let a = FooBarData {\n        value: 1u8\n    };\n    let b = a.set(42);\n    let c = b.value;\n    let d = b.return_it(true);\n    let e = b.return_it(9u64);\n    let f = FooBarData {\n        value: 1u64\n    };\n    let g = f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8));\n    let h = FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    );\n    let i = OtherData {\n        a: true,\n        b: false,\n    };\n    let j = OtherData {\n        a: 10u32,\n        b: 11u32,\n    };\n    let k = j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8));\n    let l = FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    );\n    let m = MyPoint {\n        x: 10u64,\n        y: 10u64,\n    };\n    let n = m.my_double(100);\n    let o = m.my_triple(100);\n    let p = MyU64 {\n        inner: 30u64\n    };\n    let q = p.my_triple(1);\n\n    if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }\n}",
        },
    },
]
