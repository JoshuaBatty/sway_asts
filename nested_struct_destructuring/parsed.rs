[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08f5aad80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                            ),
                            start: 50,
                            end: 54,
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
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 74,
                                                    end: 89,
                                                    as_str(): "tuple_in_struct",
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 105,
                                                                        as_str(): "TupleInStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 105,
                                                                as_str(): "TupleInStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 116,
                                                                        end: 128,
                                                                        as_str(): "nested_tuple",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Tuple(
                                                                        [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    U64(
                                                                                        42,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 131,
                                                                                    end: 136,
                                                                                    as_str(): "42u64",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Tuple(
                                                                                    [
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                U32(
                                                                                                    42,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 139,
                                                                                                end: 144,
                                                                                                as_str(): "42u32",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Tuple(
                                                                                                [
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Boolean(
                                                                                                                true,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 147,
                                                                                                            end: 151,
                                                                                                            as_str(): "true",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            String(
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 154,
                                                                                                                    end: 156,
                                                                                                                    as_str(): "ok",
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 153,
                                                                                                            end: 157,
                                                                                                            as_str(): "\"ok\"",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 146,
                                                                                                end: 158,
                                                                                                as_str(): "(true, \"ok\")",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 138,
                                                                                    end: 160,
                                                                                    as_str(): "(42u32, (true, \"ok\") )",
                                                                                },
                                                                            },
                                                                        ],
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 130,
                                                                        end: 162,
                                                                        as_str(): "(42u64, (42u32, (true, \"ok\") ) )",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 116,
                                                                    end: 162,
                                                                    as_str(): "nested_tuple: (42u64, (42u32, (true, \"ok\") ) )",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 169,
                                                    as_str(): "TupleInStruct {\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 70,
                                    end: 170,
                                    as_str(): "let tuple_in_struct = TupleInStruct {\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 179,
                                                    end: 192,
                                                    as_str(): "TupleInStruct",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 232,
                                                            end: 247,
                                                            as_str(): "tuple_in_struct",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 232,
                                                    end: 247,
                                                    as_str(): "tuple_in_struct",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_1",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_1",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 179,
                                                                        end: 192,
                                                                        as_str(): "TupleInStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 195,
                                                                end: 207,
                                                                as_str(): "nested_tuple",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 210,
                                                    end: 211,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_1",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 248,
                                                                        as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 248,
                                                            as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_1",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 248,
                                                                        as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 248,
                                                            as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 215,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_2",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 248,
                                                                        as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 248,
                                                            as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_3",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_2",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 248,
                                                                        as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 248,
                                                            as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 218,
                                                    end: 219,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_3",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 248,
                                                                        as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 248,
                                                            as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 221,
                                                    end: 222,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_3",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 248,
                                                                        as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 175,
                                                                end: 248,
                                                                as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                            },
                                                        },
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 248,
                                                            as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 175,
                                                    end: 248,
                                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 248,
                                    as_str(): "let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 258,
                                                    end: 273,
                                                    as_str(): "struct_in_tuple",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Struct(
                                                                StructExpression {
                                                                    call_path_binding: TypeBinding {
                                                                        inner: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 277,
                                                                                    end: 282,
                                                                                    as_str(): "Point",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 277,
                                                                            end: 282,
                                                                            as_str(): "Point",
                                                                        },
                                                                    },
                                                                    fields: [
                                                                        StructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 285,
                                                                                    end: 286,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        2,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 288,
                                                                                    end: 289,
                                                                                    as_str(): "2",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 285,
                                                                                end: 289,
                                                                                as_str(): "x: 2",
                                                                            },
                                                                        },
                                                                        StructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 291,
                                                                                    end: 292,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        4,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 294,
                                                                                    end: 295,
                                                                                    as_str(): "4",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 291,
                                                                                end: 295,
                                                                                as_str(): "y: 4",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 277,
                                                                end: 298,
                                                                as_str(): "Point { x: 2, y: 4, }",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: Struct(
                                                                StructExpression {
                                                                    call_path_binding: TypeBinding {
                                                                        inner: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 300,
                                                                                    end: 305,
                                                                                    as_str(): "Point",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 300,
                                                                            end: 305,
                                                                            as_str(): "Point",
                                                                        },
                                                                    },
                                                                    fields: [
                                                                        StructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 308,
                                                                                    end: 309,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 311,
                                                                                    end: 312,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 308,
                                                                                end: 312,
                                                                                as_str(): "x: 3",
                                                                            },
                                                                        },
                                                                        StructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 314,
                                                                                    end: 315,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        6,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 317,
                                                                                    end: 318,
                                                                                    as_str(): "6",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 314,
                                                                                end: 318,
                                                                                as_str(): "y: 6",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 300,
                                                                end: 320,
                                                                as_str(): "Point { x: 3, y: 6 }",
                                                            },
                                                        },
                                                    ],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 276,
                                                    end: 321,
                                                    as_str(): "(Point { x: 2, y: 4, }, Point { x: 3, y: 6 })",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 254,
                                    end: 322,
                                    as_str(): "let struct_in_tuple = (Point { x: 2, y: 4, }, Point { x: 3, y: 6 });",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__tuple_4",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 382,
                                                            end: 397,
                                                            as_str(): "struct_in_tuple",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 382,
                                                    end: 397,
                                                    as_str(): "struct_in_tuple",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_2",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 332,
                                                    end: 337,
                                                    as_str(): "Point",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_4",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 327,
                                                                        end: 398,
                                                                        as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 398,
                                                                as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                            },
                                                        },
                                                        index: 0,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 398,
                                                            as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 343,
                                                    end: 345,
                                                    as_str(): "x0",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_2",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 332,
                                                                        end: 337,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 398,
                                                                as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 340,
                                                                end: 341,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 350,
                                                    end: 352,
                                                    as_str(): "y0",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_2",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 332,
                                                                        end: 337,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 398,
                                                                as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 347,
                                                                end: 348,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_3",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 356,
                                                    end: 361,
                                                    as_str(): "Point",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: TupleIndex(
                                                    TupleIndexExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "__tuple_4",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 327,
                                                                        end: 398,
                                                                        as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 398,
                                                                as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                            },
                                                        },
                                                        index: 1,
                                                        index_span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 398,
                                                            as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 367,
                                                    end: 369,
                                                    as_str(): "x1",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_3",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 356,
                                                                        end: 361,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 398,
                                                                as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 364,
                                                                end: 365,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 374,
                                                    end: 376,
                                                    as_str(): "y1",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_3",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 356,
                                                                        end: 361,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 398,
                                                                as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 371,
                                                                end: 372,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 398,
                                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 398,
                                    as_str(): "let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 408,
                                                    end: 414,
                                                    as_str(): "point1",
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 417,
                                                                        end: 422,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 417,
                                                                end: 422,
                                                                as_str(): "Point",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 425,
                                                                        end: 426,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 428,
                                                                        end: 429,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 425,
                                                                    end: 429,
                                                                    as_str(): "x: 0",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 431,
                                                                        end: 432,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 434,
                                                                        end: 435,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 431,
                                                                    end: 435,
                                                                    as_str(): "y: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 417,
                                                    end: 437,
                                                    as_str(): "Point { x: 0, y: 0 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 404,
                                    end: 438,
                                    as_str(): "let point1 = Point { x: 0, y: 0 };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 447,
                                                    end: 453,
                                                    as_str(): "point2",
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 456,
                                                                        end: 461,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 456,
                                                                end: 461,
                                                                as_str(): "Point",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 464,
                                                                        end: 465,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 467,
                                                                        end: 468,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 464,
                                                                    end: 468,
                                                                    as_str(): "x: 1",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 470,
                                                                        end: 471,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 473,
                                                                        end: 474,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 470,
                                                                    end: 474,
                                                                    as_str(): "y: 1",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 456,
                                                    end: 476,
                                                    as_str(): "Point { x: 1, y: 1 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 443,
                                    end: 477,
                                    as_str(): "let point2 = Point { x: 1, y: 1 };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 486,
                                                    end: 490,
                                                    as_str(): "line",
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 493,
                                                                        end: 497,
                                                                        as_str(): "Line",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 493,
                                                                end: 497,
                                                                as_str(): "Line",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 500,
                                                                        end: 502,
                                                                        as_str(): "p1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 504,
                                                                                end: 510,
                                                                                as_str(): "point1",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 504,
                                                                        end: 510,
                                                                        as_str(): "point1",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 500,
                                                                    end: 510,
                                                                    as_str(): "p1: point1",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 512,
                                                                        end: 514,
                                                                        as_str(): "p2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 516,
                                                                                end: 522,
                                                                                as_str(): "point2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 516,
                                                                        end: 522,
                                                                        as_str(): "point2",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 512,
                                                                    end: 522,
                                                                    as_str(): "p2: point2",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 493,
                                                    end: 524,
                                                    as_str(): "Line { p1: point1, p2: point2 }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 482,
                                    end: 525,
                                    as_str(): "let line = Line { p1: point1, p2: point2 };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_4",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 534,
                                                    end: 538,
                                                    as_str(): "Line",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 599,
                                                            end: 603,
                                                            as_str(): "line",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 599,
                                                    end: 603,
                                                    as_str(): "line",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_5",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 545,
                                                    end: 550,
                                                    as_str(): "Point",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_4",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 534,
                                                                        end: 538,
                                                                        as_str(): "Line",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 604,
                                                                as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 541,
                                                                end: 543,
                                                                as_str(): "p1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 530,
                                                    end: 604,
                                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 556,
                                                    end: 558,
                                                    as_str(): "x2",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_5",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 545,
                                                                        end: 550,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 604,
                                                                as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 553,
                                                                end: 554,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 530,
                                                    end: 604,
                                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 563,
                                                    end: 565,
                                                    as_str(): "y2",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_5",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 545,
                                                                        end: 550,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 604,
                                                                as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 560,
                                                                end: 561,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 530,
                                                    end: 604,
                                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: Some(
                                                    "__destructure_6",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 573,
                                                    end: 578,
                                                    as_str(): "Point",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_4",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 534,
                                                                        end: 538,
                                                                        as_str(): "Line",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 604,
                                                                as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 569,
                                                                end: 571,
                                                                as_str(): "p2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 530,
                                                    end: 604,
                                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 584,
                                                    end: 586,
                                                    as_str(): "x3",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_6",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 573,
                                                                        end: 578,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 604,
                                                                as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 581,
                                                                end: 582,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 530,
                                                    end: 604,
                                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 591,
                                                    end: 593,
                                                    as_str(): "y3",
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
                                                                    name_override_opt: Some(
                                                                        "__destructure_6",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 573,
                                                                        end: 578,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 530,
                                                                end: 604,
                                                                as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 588,
                                                                end: 589,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 530,
                                                    end: 604,
                                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 530,
                                    end: 604,
                                    as_str(): "let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                    ),
                                                    start: 609,
                                                    end: 611,
                                                    as_str(): "x2",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f5aad80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                            ),
                                            start: 609,
                                            end: 611,
                                            as_str(): "x2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 609,
                                    end: 611,
                                    as_str(): "x2",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe08f5aad80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                            ),
                            start: 64,
                            end: 613,
                            as_str(): "{\n    let tuple_in_struct = TupleInStruct {\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    };\n    let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;\n\n    let struct_in_tuple = (Point { x: 2, y: 4, }, Point { x: 3, y: 6 });\n    let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;\n\n    let point1 = Point { x: 0, y: 0 };\n    let point2 = Point { x: 1, y: 1 };\n    let line = Line { p1: point1, p2: point2 };\n    let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;\n    x2\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe08f5aad80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                        ),
                        start: 47,
                        end: 613,
                        as_str(): "fn main() -> u64 {\n    let tuple_in_struct = TupleInStruct {\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    };\n    let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;\n\n    let struct_in_tuple = (Point { x: 2, y: 4, }, Point { x: 3, y: 6 });\n    let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;\n\n    let point1 = Point { x: 0, y: 0 };\n    let point2 = Point { x: 1, y: 1 };\n    let line = Line { p1: point1, p2: point2 };\n    let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;\n    x2\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe08f5aad80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                        ),
                        start: 60,
                        end: 63,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f5aad80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
            ),
            start: 47,
            end: 613,
            as_str(): "fn main() -> u64 {\n    let tuple_in_struct = TupleInStruct {\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    };\n    let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;\n\n    let struct_in_tuple = (Point { x: 2, y: 4, }, Point { x: 3, y: 6 });\n    let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;\n\n    let point1 = Point { x: 0, y: 0 };\n    let point2 = Point { x: 1, y: 1 };\n    let line = Line { p1: point1, p2: point2 };\n    let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;\n    x2\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08f5aad80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                            ),
                            start: 622,
                            end: 627,
                            as_str(): "Point",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 634,
                                    end: 635,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 634,
                                end: 640,
                                as_str(): "x: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 637,
                                end: 640,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 646,
                                    end: 647,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 646,
                                end: 652,
                                as_str(): "y: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 649,
                                end: 652,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe08f5aad80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                        ),
                        start: 615,
                        end: 655,
                        as_str(): "struct Point {\n    x: u64,\n    y: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f5aad80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
            ),
            start: 615,
            end: 655,
            as_str(): "struct Point {\n    x: u64,\n    y: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08f5aad80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                            ),
                            start: 664,
                            end: 668,
                            as_str(): "Line",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 675,
                                    end: 677,
                                    as_str(): "p1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 679,
                                        end: 684,
                                        as_str(): "Point",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 675,
                                end: 684,
                                as_str(): "p1: Point",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 679,
                                end: 684,
                                as_str(): "Point",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 690,
                                    end: 692,
                                    as_str(): "p2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 694,
                                        end: 699,
                                        as_str(): "Point",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 690,
                                end: 699,
                                as_str(): "p2: Point",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 694,
                                end: 699,
                                as_str(): "Point",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe08f5aad80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                        ),
                        start: 657,
                        end: 702,
                        as_str(): "struct Line {\n    p1: Point,\n    p2: Point,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f5aad80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
            ),
            start: 657,
            end: 702,
            as_str(): "struct Line {\n    p1: Point,\n    p2: Point,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08f5aad80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                            ),
                            start: 711,
                            end: 724,
                            as_str(): "TupleInStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08f5aad80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                    ),
                                    start: 731,
                                    end: 743,
                                    as_str(): "nested_tuple",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [
                                    TypeArgument {
                                        type_id: TypeId(
                                            21,
                                        ),
                                        initial_type_id: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f5aad80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                            ),
                                            start: 746,
                                            end: 749,
                                            as_str(): "u64",
                                        },
                                    },
                                    TypeArgument {
                                        type_id: TypeId(
                                            7251,
                                        ),
                                        initial_type_id: TypeId(
                                            7251,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08f5aad80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                            ),
                                            start: 751,
                                            end: 773,
                                            as_str(): "(u32, (bool, str[2]) )",
                                        },
                                    },
                                ],
                            ),
                            span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 731,
                                end: 775,
                                as_str(): "nested_tuple: (u64, (u32, (bool, str[2]) ) )",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe08f5aad80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                ),
                                start: 745,
                                end: 775,
                                as_str(): "(u64, (u32, (bool, str[2]) ) )",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe08f5aad80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                        ),
                        start: 704,
                        end: 778,
                        as_str(): "struct TupleInStruct {\n    nested_tuple: (u64, (u32, (bool, str[2]) ) ),\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08f5aad80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
            ),
            start: 704,
            end: 778,
            as_str(): "struct TupleInStruct {\n    nested_tuple: (u64, (u32, (bool, str[2]) ) ),\n}",
        },
    },
]
