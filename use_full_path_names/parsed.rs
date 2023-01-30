[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 9,
                    end: 26,
                    as_str(): "dep a_dependency;",
                },
                _path_span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 13,
                    end: 25,
                    as_str(): "a_dependency",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007f8a1d1ebd20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
            ),
            start: 9,
            end: 26,
            as_str(): "dep a_dependency;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 27,
                    end: 44,
                    as_str(): "dep b_dependency;",
                },
                _path_span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 31,
                    end: 43,
                    as_str(): "b_dependency",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007f8a1d1ebd20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
            ),
            start: 27,
            end: 44,
            as_str(): "dep b_dependency;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 45,
                    end: 62,
                    as_str(): "dep c_dependency;",
                },
                _path_span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 49,
                    end: 61,
                    as_str(): "c_dependency",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007f8a1d1ebd20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
            ),
            start: 45,
            end: 62,
            as_str(): "dep c_dependency;",
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
                            src (ptr): 0x00007f8a1d1ebd20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): "x",
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
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 95,
                                                                            end: 98,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                        ),
                                                                        start: 100,
                                                                        end: 103,
                                                                        as_str(): "Foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 100,
                                                                end: 103,
                                                                as_str(): "Foo",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 117,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                        ),
                                                                        start: 119,
                                                                        end: 123,
                                                                        as_str(): "1u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 123,
                                                                    as_str(): "foo: 1u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 95,
                                                    end: 130,
                                                    as_str(): "foo::Foo {\n        foo: 1u32,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 87,
                                    end: 131,
                                    as_str(): "let x = foo::Foo {\n        foo: 1u32,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 140,
                                                    end: 141,
                                                    as_str(): "y",
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
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 147,
                                                                            as_str(): "bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 149,
                                                                                end: 152,
                                                                                as_str(): "Bar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 149,
                                                                            end: 152,
                                                                            as_str(): "Bar",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 154,
                                                                            end: 157,
                                                                            as_str(): "Baz",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 144,
                                                                end: 157,
                                                                as_str(): "bar::Bar::Baz",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                    ),
                                                                    start: 158,
                                                                    end: 162,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 163,
                                                    as_str(): "bar::Bar::Baz(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 136,
                                    end: 164,
                                    as_str(): "let y = bar::Bar::Baz(true);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 173,
                                                    end: 174,
                                                    as_str(): "z",
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
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 179,
                                                                            end: 182,
                                                                            as_str(): "bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                                ),
                                                                                start: 184,
                                                                                end: 187,
                                                                                as_str(): "Bar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 187,
                                                                            as_str(): "Bar",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d1ebd20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                            ),
                                                                            start: 189,
                                                                            end: 192,
                                                                            as_str(): "Baz",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: true,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d1ebd20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 192,
                                                                as_str(): "bar::Bar::Baz",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        false,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                    ),
                                                                    start: 193,
                                                                    end: 198,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 177,
                                                    end: 199,
                                                    as_str(): "::bar::Bar::Baz(false)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 169,
                                    end: 200,
                                    as_str(): "let z = ::bar::Bar::Baz(false);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
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
                                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 208,
                                                                        as_str(): "baz",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                    ),
                                                                    start: 205,
                                                                    end: 208,
                                                                    as_str(): "baz",
                                                                },
                                                            },
                                                            suffix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                                    ),
                                                                    start: 210,
                                                                    end: 218,
                                                                    as_str(): "return_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1d1ebd20,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                        ),
                                                        start: 205,
                                                        end: 218,
                                                        as_str(): "baz::return_1",
                                                    },
                                                },
                                                args: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 205,
                                            end: 220,
                                            as_str(): "baz::return_1()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 205,
                                    end: 220,
                                    as_str(): "baz::return_1()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a1d1ebd20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                            ),
                            start: 81,
                            end: 222,
                            as_str(): "{\n    let x = foo::Foo {\n        foo: 1u32,\n    };\n    let y = bar::Bar::Baz(true);\n    let z = ::bar::Bar::Baz(false);\n    baz::return_1()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a1d1ebd20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                        ),
                        start: 64,
                        end: 222,
                        as_str(): "fn main() -> u64 {\n    let x = foo::Foo {\n        foo: 1u32,\n    };\n    let y = bar::Bar::Baz(true);\n    let z = ::bar::Bar::Baz(false);\n    baz::return_1()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a1d1ebd20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                        ),
                        start: 77,
                        end: 80,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a1d1ebd20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
            ),
            start: 64,
            end: 222,
            as_str(): "fn main() -> u64 {\n    let x = foo::Foo {\n        foo: 1u32,\n    };\n    let y = bar::Bar::Baz(true);\n    let z = ::bar::Bar::Baz(false);\n    baz::return_1()\n}",
        },
    },
]
