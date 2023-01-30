[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                src (ptr): 0x00007fe0629e5bd0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                ),
                                start: 26,
                                end: 36,
                                as_str(): "value: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0629e5bd0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                        src (ptr): 0x00007fe0629e5bd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                        ),
                        start: 9,
                        end: 38,
                        as_str(): "struct Foo {\n    value: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0629e5bd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 43,
                            end: 50,
                            as_str(): "mut_foo",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Reassignment(
                                            ReassignmentExpression {
                                                lhs: VariableExpression(
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 75,
                                                                    end: 78,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 78,
                                                            as_str(): "foo",
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
                                                                            src (ptr): 0x00007fe0629e5bd0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                            ),
                                                                            start: 81,
                                                                            end: 84,
                                                                            as_str(): "Foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 84,
                                                                    as_str(): "Foo",
                                                                },
                                                            },
                                                            fields: [
                                                                StructExpressionField {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0629e5bd0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                            ),
                                                                            start: 87,
                                                                            end: 92,
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
                                                                            src (ptr): 0x00007fe0629e5bd0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                            ),
                                                                            start: 94,
                                                                            end: 96,
                                                                            as_str(): "10",
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 87,
                                                                        end: 96,
                                                                        as_str(): "value: 10",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 81,
                                                        end: 98,
                                                        as_str(): "Foo { value: 10 }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 98,
                                            as_str(): "foo = Foo { value: 10 }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 75,
                                    end: 98,
                                    as_str(): "foo = Foo { value: 10 }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 69,
                            end: 101,
                            as_str(): "{\n    foo = Foo { value: 10 };\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 62,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: true,
                            is_mutable: true,
                            mutability_span: Span {
                                src (ptr): 0x00007fe0629e5bd0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                ),
                                start: 51,
                                end: 58,
                                as_str(): "ref mut",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 67,
                                        as_str(): "Foo",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0629e5bd0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                ),
                                start: 64,
                                end: 67,
                                as_str(): "Foo",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0629e5bd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                        ),
                        start: 40,
                        end: 101,
                        as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo = Foo { value: 10 };\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0629e5bd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                        ),
                        start: 40,
                        end: 68,
                        as_str(): "fn mut_foo(ref mut foo: Foo)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0629e5bd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
            ),
            start: 40,
            end: 101,
            as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo = Foo { value: 10 };\n}",
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
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 106,
                            end: 110,
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
                                                    src (ptr): 0x00007fe0629e5bd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                    ),
                                                    start: 134,
                                                    end: 137,
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
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 151,
                                                                        as_str(): "value",
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
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 153,
                                                                        end: 154,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 146,
                                                                    end: 154,
                                                                    as_str(): "value: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0629e5bd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                    ),
                                                    start: 140,
                                                    end: 156,
                                                    as_str(): "Foo { value: 0 }",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 126,
                                    end: 157,
                                    as_str(): "let mut foo = Foo { value: 0 };",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 169,
                                                                as_str(): "mut_foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 162,
                                                        end: 169,
                                                        as_str(): "mut_foo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 170,
                                                                    end: 173,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 173,
                                                            as_str(): "foo",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 162,
                                            end: 174,
                                            as_str(): "mut_foo(foo)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 174,
                                    as_str(): "mut_foo(foo)",
                                },
                            },
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
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 183,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 180,
                                                        end: 183,
                                                        as_str(): "foo",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 184,
                                                        end: 189,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 180,
                                            end: 189,
                                            as_str(): "foo.value",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 180,
                                    end: 189,
                                    as_str(): "foo.value",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 120,
                            end: 191,
                            as_str(): "{\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0629e5bd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                        ),
                        start: 103,
                        end: 191,
                        as_str(): "fn main() -> u64 {\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0629e5bd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                        ),
                        start: 116,
                        end: 119,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0629e5bd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
            ),
            start: 103,
            end: 191,
            as_str(): "fn main() -> u64 {\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
        },
    },
]
