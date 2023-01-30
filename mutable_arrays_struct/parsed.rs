[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe096cd2c70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
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
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 26,
                                    end: 31,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe096cd2c70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                ),
                                start: 26,
                                end: 36,
                                as_str(): "value: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe096cd2c70,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                ),
                                start: 33,
                                end: 36,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe096cd2c70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                        ),
                        start: 9,
                        end: 38,
                        as_str(): "struct Foo {\n    value: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe096cd2c70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
            ),
            start: 9,
            end: 38,
            as_str(): "struct Foo {\n    value: u64\n}",
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
                            src (ptr): 0x00007fe096cd2c70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                            ),
                            start: 43,
                            end: 47,
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
                                                    src (ptr): 0x00007fe096cd2c70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 79,
                                                    as_str(): "my_array",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        0,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        0,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 82,
                                                        end: 85,
                                                        as_str(): "Foo",
                                                    },
                                                },
                                                Length {
                                                    val: 1,
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 87,
                                                        end: 88,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe096cd2c70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                    ),
                                                    start: 81,
                                                    end: 89,
                                                    as_str(): "[Foo; 1]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Struct(
                                                                    StructExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 93,
                                                                                        end: 96,
                                                                                        as_str(): "Foo",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe096cd2c70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                ),
                                                                                start: 93,
                                                                                end: 96,
                                                                                as_str(): "Foo",
                                                                            },
                                                                        },
                                                                        fields: [
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 97,
                                                                                        end: 102,
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
                                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 104,
                                                                                        end: 106,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 97,
                                                                                    end: 106,
                                                                                    as_str(): "value: 10",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 93,
                                                                    end: 107,
                                                                    as_str(): "Foo{value: 10}",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe096cd2c70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 108,
                                                    as_str(): "[Foo{value: 10}]",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 63,
                                    end: 109,
                                    as_str(): "let mut my_array: [Foo; 1] = [Foo{value: 10}];",
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
                                                                                src (ptr): 0x00007fe096cd2c70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                ),
                                                                                start: 114,
                                                                                end: 122,
                                                                                as_str(): "my_array",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 122,
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
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 123,
                                                                        end: 124,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 125,
                                                            as_str(): "my_array[0]",
                                                        },
                                                    },
                                                ),
                                                rhs: Expression {
                                                    kind: Struct(
                                                        StructExpression {
                                                            call_path_binding: TypeBinding {
                                                                inner: CallPath {
                                                                    prefixes: [],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                            ),
                                                                            start: 128,
                                                                            end: 131,
                                                                            as_str(): "Foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 131,
                                                                    as_str(): "Foo",
                                                                },
                                                            },
                                                            fields: [
                                                                StructExpressionField {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                            ),
                                                                            start: 132,
                                                                            end: 137,
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
                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                            ),
                                                                            start: 139,
                                                                            end: 141,
                                                                            as_str(): "20",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 132,
                                                                        end: 141,
                                                                        as_str(): "value: 20",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 128,
                                                        end: 142,
                                                        as_str(): "Foo{value: 20}",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 114,
                                            end: 142,
                                            as_str(): "my_array[0] = Foo{value: 20}",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 142,
                                    as_str(): "my_array[0] = Foo{value: 20}",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Subfield(
                                            SubfieldExpression {
                                                prefix: Expression {
                                                    kind: ArrayIndex(
                                                        ArrayIndexExpression {
                                                            prefix: Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                            ),
                                                                            start: 148,
                                                                            end: 156,
                                                                            as_str(): "my_array",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 148,
                                                                    end: 156,
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
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 157,
                                                                    end: 158,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 148,
                                                        end: 159,
                                                        as_str(): "my_array[0]",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 160,
                                                        end: 165,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 148,
                                            end: 165,
                                            as_str(): "my_array[0].value",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe096cd2c70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                    ),
                                    start: 148,
                                    end: 165,
                                    as_str(): "my_array[0].value",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe096cd2c70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                            ),
                            start: 57,
                            end: 167,
                            as_str(): "{\n    let mut my_array: [Foo; 1] = [Foo{value: 10}];\n    my_array[0] = Foo{value: 20};\n    my_array[0].value\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe096cd2c70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                        ),
                        start: 40,
                        end: 167,
                        as_str(): "fn main() -> u64 {\n    let mut my_array: [Foo; 1] = [Foo{value: 10}];\n    my_array[0] = Foo{value: 20};\n    my_array[0].value\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe096cd2c70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                        ),
                        start: 53,
                        end: 56,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe096cd2c70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
            ),
            start: 40,
            end: 167,
            as_str(): "fn main() -> u64 {\n    let mut my_array: [Foo; 1] = [Foo{value: 10}];\n    my_array[0] = Foo{value: 20};\n    my_array[0].value\n}",
        },
    },
]
