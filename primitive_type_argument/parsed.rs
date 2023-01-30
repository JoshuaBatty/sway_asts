[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 10,
                    end: 18,
                    as_str(): "dep foo;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 14,
                    end: 17,
                    as_str(): "foo",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 10,
            end: 18,
            as_str(): "dep foo;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe05b2d99e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                            ),
                            start: 27,
                            end: 28,
                            as_str(): "S",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [
                        T: TypeId(31636),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe05b2d99e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                        ),
                        start: 20,
                        end: 35,
                        as_str(): "struct S<T> { }",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 20,
            end: 35,
            as_str(): "struct S<T> { }",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [
                        T: TypeId(31638),
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe05b2d99e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                ),
                                start: 45,
                                end: 46,
                                as_str(): "S",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        31637,
                                    ),
                                    initial_type_id: TypeId(
                                        31637,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 47,
                                        end: 48,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe05b2d99e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                        ),
                        start: 45,
                        end: 49,
                        as_str(): "S<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 57,
                                    end: 58,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        5,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 79,
                                                    as_str(): "5",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 79,
                                            as_str(): "5",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 72,
                                    end: 83,
                                    as_str(): "{\n    5\n  }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 63,
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
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 63,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe05b2d99e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                ),
                                start: 54,
                                end: 83,
                                as_str(): "fn f(self) -> u64 {\n    5\n  }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe05b2d99e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                ),
                                start: 68,
                                end: 71,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe05b2d99e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                        ),
                        start: 37,
                        end: 85,
                        as_str(): "impl<T> S<T> {\n  fn f(self) -> u64 {\n    5\n  }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 37,
            end: 85,
            as_str(): "impl<T> S<T> {\n  fn f(self) -> u64 {\n    5\n  }\n}",
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
                            src (ptr): 0x00007fe05b2d99e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                            ),
                            start: 90,
                            end: 94,
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
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 112,
                                                    end: 113,
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 116,
                                                                        end: 117,
                                                                        as_str(): "S",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 120,
                                                                        end: 123,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 124,
                                                                as_str(): "S::<u64>",
                                                            },
                                                        },
                                                        fields: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 128,
                                                    as_str(): "S::<u64> { }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 108,
                                    end: 129,
                                    as_str(): "let a = S::<u64> { };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 136,
                                                    end: 137,
                                                    as_str(): "b",
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
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 143,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 145,
                                                                            end: 148,
                                                                            as_str(): "baz",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 163,
                                                                        as_str(): "ExampleStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 166,
                                                                        end: 169,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 171,
                                                                        end: 175,
                                                                        as_str(): "bool",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 150,
                                                                end: 176,
                                                                as_str(): "ExampleStruct::<u64, bool>",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 179,
                                                                        end: 186,
                                                                        as_str(): "a_field",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 192,
                                                                        as_str(): "5u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 179,
                                                                    end: 192,
                                                                    as_str(): "a_field: 5u64",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 201,
                                                                        as_str(): "b_field",
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 203,
                                                                        end: 207,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 207,
                                                                    as_str(): "b_field: true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 140,
                                                    end: 209,
                                                    as_str(): "foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 132,
                                    end: 210,
                                    as_str(): "let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };",
                                },
                            },
                            AstNode {
                                content: UseStatement(
                                    UseStatement {
                                        call_path: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 217,
                                                    end: 220,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 222,
                                                    end: 225,
                                                    as_str(): "baz",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        import_type: Item(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 227,
                                                    end: 240,
                                                    as_str(): "ExampleStruct",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        is_absolute: false,
                                        alias: None,
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 213,
                                    end: 241,
                                    as_str(): "use foo::baz::ExampleStruct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 248,
                                                    end: 249,
                                                    as_str(): "c",
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
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 252,
                                                                            end: 255,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 257,
                                                                            end: 260,
                                                                            as_str(): "baz",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 262,
                                                                            end: 266,
                                                                            as_str(): "quux",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 268,
                                                                        end: 272,
                                                                        as_str(): "Quux",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 275,
                                                                        end: 278,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 280,
                                                                        end: 284,
                                                                        as_str(): "bool",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        31639,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        31639,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 286,
                                                                        end: 310,
                                                                        as_str(): "ExampleStruct<u64, bool>",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 312,
                                                                        end: 315,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        31640,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        31640,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 317,
                                                                        end: 323,
                                                                        as_str(): "str[3]",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 325,
                                                                        end: 328,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 268,
                                                                end: 329,
                                                                as_str(): "Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64>",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 332,
                                                                        end: 333,
                                                                        as_str(): "a",
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 335,
                                                                        end: 337,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 332,
                                                                    end: 337,
                                                                    as_str(): "a: 10",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 339,
                                                                        end: 340,
                                                                        as_str(): "b",
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 342,
                                                                        end: 346,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 339,
                                                                    end: 346,
                                                                    as_str(): "b: true",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 348,
                                                                        end: 349,
                                                                        as_str(): "c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 351,
                                                                                end: 352,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 351,
                                                                        end: 352,
                                                                        as_str(): "b",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 348,
                                                                    end: 352,
                                                                    as_str(): "c: b",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 354,
                                                                        end: 355,
                                                                        as_str(): "d",
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 357,
                                                                        end: 359,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 354,
                                                                    end: 359,
                                                                    as_str(): "d: 10",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 361,
                                                                        end: 362,
                                                                        as_str(): "e",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        String(
                                                                            Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 365,
                                                                                end: 368,
                                                                                as_str(): "foo",
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 364,
                                                                        end: 369,
                                                                        as_str(): "\"foo\"",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 361,
                                                                    end: 369,
                                                                    as_str(): "e: \"foo\"",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 371,
                                                                        end: 372,
                                                                        as_str(): "f",
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 374,
                                                                        end: 376,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 371,
                                                                    end: 376,
                                                                    as_str(): "f: 10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 252,
                                                    end: 378,
                                                    as_str(): "foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 244,
                                    end: 379,
                                    as_str(): "let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 392,
                                                                        as_str(): "f",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 391,
                                                                end: 392,
                                                                as_str(): "f",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 389,
                                                                            end: 390,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 389,
                                                                    end: 390,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 389,
                                                    end: 394,
                                                    as_str(): "a.f()",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 382,
                                            end: 394,
                                            as_str(): "return a.f()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 382,
                                    end: 394,
                                    as_str(): "return a.f()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05b2d99e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                            ),
                            start: 104,
                            end: 397,
                            as_str(): "{\n  let a = S::<u64> { };\n  let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };\n  use foo::baz::ExampleStruct;\n  let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };\n  return a.f();\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05b2d99e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                        ),
                        start: 87,
                        end: 397,
                        as_str(): "fn main() -> u64 {\n  let a = S::<u64> { };\n  let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };\n  use foo::baz::ExampleStruct;\n  let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };\n  return a.f();\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05b2d99e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                        ),
                        start: 100,
                        end: 103,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 87,
            end: 397,
            as_str(): "fn main() -> u64 {\n  let a = S::<u64> { };\n  let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };\n  use foo::baz::ExampleStruct;\n  let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };\n  return a.f();\n}",
        },
    },
]
