[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb13db7f560,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                    ),
                    start: 46,
                    end: 63,
                    as_str(): "dep a_dependency;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb13db7f560,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                    ),
                    start: 50,
                    end: 62,
                    as_str(): "a_dependency",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb13db7f560,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
            ),
            start: 46,
            end: 63,
            as_str(): "dep a_dependency;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 69,
                            end: 72,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 74,
                            end: 77,
                            as_str(): "Foo",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: Some(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 81,
                            end: 86,
                            as_str(): "MyFoo",
                        },
                        is_raw_ident: false,
                    },
                ),
            },
        ),
        span: Span {
            src (ptr): 0x00007fb13db7f560,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
            ),
            start: 65,
            end: 87,
            as_str(): "use foo::Foo as MyFoo;",
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
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 92,
                            end: 96,
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
                                                    src (ptr): 0x00007fb13db7f560,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 119,
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
                                                                        src (ptr): 0x00007fb13db7f560,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                        ),
                                                                        start: 122,
                                                                        end: 127,
                                                                        as_str(): "MyFoo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13db7f560,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 127,
                                                                as_str(): "MyFoo",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13db7f560,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                        ),
                                                                        start: 138,
                                                                        end: 141,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13db7f560,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                        ),
                                                                        start: 143,
                                                                        end: 145,
                                                                        as_str(): "42",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13db7f560,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                    ),
                                                                    start: 138,
                                                                    end: 145,
                                                                    as_str(): "foo: 42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13db7f560,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 152,
                                                    as_str(): "MyFoo {\n        foo: 42,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13db7f560,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                    ),
                                    start: 112,
                                    end: 153,
                                    as_str(): "let foo = MyFoo {\n        foo: 42,\n    };",
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
                                                                src (ptr): 0x00007fb13db7f560,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                ),
                                                                start: 158,
                                                                end: 161,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb13db7f560,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                        ),
                                                        start: 158,
                                                        end: 161,
                                                        as_str(): "foo",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb13db7f560,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                        ),
                                                        start: 162,
                                                        end: 165,
                                                        as_str(): "foo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 158,
                                            end: 165,
                                            as_str(): "foo.foo",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13db7f560,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 165,
                                    as_str(): "foo.foo",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 106,
                            end: 167,
                            as_str(): "{\n    let foo = MyFoo {\n        foo: 42,\n    };\n    foo.foo\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb13db7f560,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                        ),
                        start: 89,
                        end: 167,
                        as_str(): "fn main() -> u64 {\n    let foo = MyFoo {\n        foo: 42,\n    };\n    foo.foo\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb13db7f560,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                        ),
                        start: 102,
                        end: 105,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13db7f560,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
            ),
            start: 89,
            end: 167,
            as_str(): "fn main() -> u64 {\n    let foo = MyFoo {\n        foo: 42,\n    };\n    foo.foo\n}",
        },
    },
]
