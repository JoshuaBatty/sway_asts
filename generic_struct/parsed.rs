[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 16,
                            end: 19,
                            as_str(): "Foo",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 27,
                                    end: 28,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 31,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0f9b4a470,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                ),
                                start: 27,
                                end: 31,
                                as_str(): "a: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f9b4a470,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                ),
                                start: 30,
                                end: 31,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7249),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0f9b4a470,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                        ),
                        start: 9,
                        end: 34,
                        as_str(): "struct Foo<T> {\n  a: T,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f9b4a470,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
            ),
            start: 9,
            end: 34,
            as_str(): "struct Foo<T> {\n  a: T,\n}",
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
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 39,
                            end: 44,
                            as_str(): "get_a",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Subfield(
                                            SubfieldExpression {
                                                prefix: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 70,
                                                                end: 73,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 70,
                                                        end: 73,
                                                        as_str(): "foo",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 74,
                                                        end: 75,
                                                        as_str(): "a",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 75,
                                            as_str(): "foo.a",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 70,
                                    end: 75,
                                    as_str(): "foo.a",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 66,
                            end: 77,
                            as_str(): "{\n  foo.a\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 48,
                                    end: 51,
                                    as_str(): "foo",
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
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 56,
                                        as_str(): "Foo",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                7250,
                                            ),
                                            initial_type_id: TypeId(
                                                7250,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0f9b4a470,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                ),
                                                start: 57,
                                                end: 58,
                                                as_str(): "V",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f9b4a470,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                ),
                                start: 53,
                                end: 59,
                                as_str(): "Foo<V>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0f9b4a470,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                        ),
                        start: 36,
                        end: 77,
                        as_str(): "fn get_a<V>(foo: Foo<V>) -> V {\n  foo.a\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0f9b4a470,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                ),
                                start: 64,
                                end: 65,
                                as_str(): "V",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        V: TypeId(7251),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0f9b4a470,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                        ),
                        start: 64,
                        end: 65,
                        as_str(): "V",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f9b4a470,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
            ),
            start: 36,
            end: 77,
            as_str(): "fn get_a<V>(foo: Foo<V>) -> V {\n  foo.a\n}",
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
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 82,
                            end: 86,
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
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 108,
                                                    as_str(): "foo",
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
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 111,
                                                                        end: 114,
                                                                        as_str(): "Foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 111,
                                                                end: 114,
                                                                as_str(): "Foo",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 118,
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
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 120,
                                                                        end: 124,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f9b4a470,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 124,
                                                                    as_str(): "a: true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 111,
                                                    end: 126,
                                                    as_str(): "Foo { a: true }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 101,
                                    end: 127,
                                    as_str(): "let foo = Foo { a: true };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 134,
                                                    end: 137,
                                                    as_str(): "bar",
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
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 140,
                                                                        end: 143,
                                                                        as_str(): "Foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 140,
                                                                end: 143,
                                                                as_str(): "Foo",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 147,
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
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 149,
                                                                        end: 151,
                                                                        as_str(): "10",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f9b4a470,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                    ),
                                                                    start: 146,
                                                                    end: 151,
                                                                    as_str(): "a: 10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 140,
                                                    end: 153,
                                                    as_str(): "Foo { a: 10 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 130,
                                    end: 154,
                                    as_str(): "let bar = Foo { a: 10 };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 158,
                                                                end: 163,
                                                                as_str(): "get_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 158,
                                                        end: 163,
                                                        as_str(): "get_a",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f9b4a470,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                    ),
                                                                    start: 164,
                                                                    end: 167,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 167,
                                                            as_str(): "foo",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 158,
                                            end: 168,
                                            as_str(): "get_a(foo)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 168,
                                    as_str(): "get_a(foo)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 97,
                            end: 170,
                            as_str(): "{\n  let foo = Foo { a: true };\n  let bar = Foo { a: 10 };\n\n  get_a(foo)\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0f9b4a470,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                        ),
                        start: 79,
                        end: 170,
                        as_str(): "fn main() -> bool {\n  let foo = Foo { a: true };\n  let bar = Foo { a: 10 };\n\n  get_a(foo)\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0f9b4a470,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                        ),
                        start: 92,
                        end: 96,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f9b4a470,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
            ),
            start: 79,
            end: 170,
            as_str(): "fn main() -> bool {\n  let foo = Foo { a: true };\n  let bar = Foo { a: 10 };\n\n  get_a(foo)\n}",
        },
    },
]
