[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
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
                    attributes: {},
                    fields: [
                        StructField {
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
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0935f24b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
            ),
            start: 9,
            end: 36,
            as_str(): "struct X {\n    value: u64\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
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
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 58,
                                        end: 59,
                                        as_str(): "X",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0935f24b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
            ),
            start: 38,
            end: 62,
            as_str(): "enum Foo {\n    Bar: X,\n}",
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
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        7249,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        7249,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 106,
                                                        end: 109,
                                                        as_str(): "Foo",
                                                    },
                                                },
                                                Length {
                                                    val: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
                                                        as_str(): "1",
                                                    },
                                                },
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
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
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
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 120,
                                                                                                as_str(): "Foo",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 117,
                                                                                            end: 120,
                                                                                            as_str(): "Foo",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 122,
                                                                                            end: 125,
                                                                                            as_str(): "Bar",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
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
                                                                        args: [
                                                                            Expression {
                                                                                kind: Struct(
                                                                                    StructExpression {
                                                                                        call_path_binding: TypeBinding {
                                                                                            inner: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 126,
                                                                                                        end: 127,
                                                                                                        as_str(): "X",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
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
                                                                                        fields: [
                                                                                            StructExpressionField {
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
                                                                                                value: Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            10,
                                                                                                        ),
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
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 128,
                                                                                                    end: 137,
                                                                                                    as_str(): "value: 10",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
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
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 139,
                                                                    as_str(): "Foo::Bar(X{value: 10})",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
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
                                            is_mutable: true,
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: ArrayIndex(
                                                            ArrayIndexExpression {
                                                                prefix: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
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
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 154,
                                                                        as_str(): "my_array",
                                                                    },
                                                                },
                                                                index: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
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
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 157,
                                                            as_str(): "my_array[0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
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
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 160,
                                                                                    end: 163,
                                                                                    as_str(): "Foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 160,
                                                                                end: 163,
                                                                                as_str(): "Foo",
                                                                            },
                                                                        },
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 165,
                                                                                end: 168,
                                                                                as_str(): "Bar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    is_absolute: false,
                                                                },
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
                                                            args: [
                                                                Expression {
                                                                    kind: Struct(
                                                                        StructExpression {
                                                                            call_path_binding: TypeBinding {
                                                                                inner: CallPath {
                                                                                    prefixes: [],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 169,
                                                                                            end: 170,
                                                                                            as_str(): "X",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
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
                                                                            fields: [
                                                                                StructExpressionField {
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
                                                                                    value: Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                20,
                                                                                            ),
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
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 171,
                                                                                        end: 180,
                                                                                        as_str(): "value: 20",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
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
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 160,
                                                        end: 182,
                                                        as_str(): "Foo::Bar(X{value: 20})",
                                                    },
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
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
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
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 194,
                                                                                                end: 202,
                                                                                                as_str(): "my_array",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
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
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
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
                                                                    is_mutable: false,
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
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
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
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 216,
                                                                                                    end: 219,
                                                                                                    as_str(): "Foo",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 221,
                                                                                                end: 224,
                                                                                                as_str(): "Bar",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
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
                                                                                            start: 225,
                                                                                            end: 226,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                    },
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
                                                                                result: Expression {
                                                                                    kind: Subfield(
                                                                                        SubfieldExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 231,
                                                                                                            end: 232,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
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
                                                                                            field_to_access: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 233,
                                                                                                    end: 238,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 216,
                                                                                    end: 239,
                                                                                    as_str(): "Foo::Bar(x) => x.value,",
                                                                                },
                                                                            },
                                                                        ],
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
                                                whole_block_span: Span {
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0935f24b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                            ),
                            start: 81,
                            end: 247,
                            as_str(): "{\n    let mut my_array: [Foo; 1] = [Foo::Bar(X{value: 10})];\n    my_array[0] = Foo::Bar(X{value: 20});\n    match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0935f24b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                        ),
                        start: 64,
                        end: 247,
                        as_str(): "fn main() -> u64 {\n    let mut my_array: [Foo; 1] = [Foo::Bar(X{value: 10})];\n    my_array[0] = Foo::Bar(X{value: 20});\n    match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0935f24b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
            ),
            start: 64,
            end: 247,
            as_str(): "fn main() -> u64 {\n    let mut my_array: [Foo; 1] = [Foo::Bar(X{value: 10})];\n    my_array[0] = Foo::Bar(X{value: 20});\n    match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }\n}",
        },
    },
]
