[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb118764f50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                    ),
                    start: 46,
                    end: 63,
                    as_str(): "dep a_dependency;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb118764f50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                    ),
                    start: 50,
                    end: 62,
                    as_str(): "a_dependency",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb118764f50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
            ),
            start: 46,
            end: 63,
            as_str(): "dep a_dependency;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb118764f50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                    ),
                    start: 64,
                    end: 94,
                    as_str(): "dep nested_dependency/bar/bar;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb118764f50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                    ),
                    start: 68,
                    end: 93,
                    as_str(): "nested_dependency/bar/bar",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb118764f50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
            ),
            start: 64,
            end: 94,
            as_str(): "dep nested_dependency/bar/bar;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 100,
                            end: 103,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 105,
                            end: 108,
                            as_str(): "Foo",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb118764f50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
            ),
            start: 96,
            end: 109,
            as_str(): "use foo::Foo;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 116,
                            end: 119,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 121,
                            end: 124,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 127,
                            end: 130,
                            as_str(): "Bar",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: true,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb118764f50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
            ),
            start: 110,
            end: 155,
            as_str(): "use ::foo::bar::{Bar, double_bar::DoubleBar};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 116,
                            end: 119,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 121,
                            end: 124,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 132,
                            end: 142,
                            as_str(): "double_bar",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 144,
                            end: 153,
                            as_str(): "DoubleBar",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: true,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb118764f50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
            ),
            start: 110,
            end: 155,
            as_str(): "use ::foo::bar::{Bar, double_bar::DoubleBar};",
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
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 160,
                            end: 164,
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
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 185,
                                                    end: 188,
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
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 191,
                                                                        end: 194,
                                                                        as_str(): "Foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 191,
                                                                end: 194,
                                                                as_str(): "Foo",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 208,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        String(
                                                                            Span {
                                                                                src (ptr): 0x00007fb118764f50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                                ),
                                                                                start: 211,
                                                                                end: 214,
                                                                                as_str(): "foo",
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 210,
                                                                        end: 215,
                                                                        as_str(): "\"foo\"",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118764f50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                    ),
                                                                    start: 205,
                                                                    end: 215,
                                                                    as_str(): "foo: \"foo\"",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 191,
                                                    end: 222,
                                                    as_str(): "Foo {\n        foo: \"foo\",\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb118764f50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                    ),
                                    start: 181,
                                    end: 223,
                                    as_str(): "let foo = Foo {\n        foo: \"foo\",\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 232,
                                                    end: 234,
                                                    as_str(): "db",
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
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 239,
                                                                            end: 242,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 244,
                                                                            end: 247,
                                                                            as_str(): "bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb118764f50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                            ),
                                                                            start: 249,
                                                                            end: 259,
                                                                            as_str(): "double_bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 261,
                                                                        end: 270,
                                                                        as_str(): "DoubleBar",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: true,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 261,
                                                                end: 270,
                                                                as_str(): "DoubleBar",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 281,
                                                                        end: 282,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 284,
                                                                        end: 288,
                                                                        as_str(): "5u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118764f50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                    ),
                                                                    start: 281,
                                                                    end: 288,
                                                                    as_str(): "a: 5u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 237,
                                                    end: 295,
                                                    as_str(): "::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb118764f50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                    ),
                                    start: 228,
                                    end: 296,
                                    as_str(): "let db = ::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 305,
                                                    end: 308,
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
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 311,
                                                                        end: 314,
                                                                        as_str(): "Bar",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb118764f50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                ),
                                                                start: 311,
                                                                end: 314,
                                                                as_str(): "Bar",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 325,
                                                                        end: 326,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb118764f50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                        ),
                                                                        start: 328,
                                                                        end: 332,
                                                                        as_str(): "5u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb118764f50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                                    ),
                                                                    start: 325,
                                                                    end: 332,
                                                                    as_str(): "a: 5u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb118764f50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                                    ),
                                                    start: 311,
                                                    end: 339,
                                                    as_str(): "Bar {\n        a: 5u32,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb118764f50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 340,
                                    as_str(): "let bar = Bar {\n        a: 5u32,\n    };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                false,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb118764f50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                            ),
                                            start: 345,
                                            end: 350,
                                            as_str(): "false",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb118764f50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                                    ),
                                    start: 345,
                                    end: 350,
                                    as_str(): "false",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb118764f50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                            ),
                            start: 175,
                            end: 352,
                            as_str(): "{\n    let foo = Foo {\n        foo: \"foo\",\n    };\n    let db = ::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    };\n    let bar = Bar {\n        a: 5u32,\n    };\n    false\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb118764f50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                        ),
                        start: 157,
                        end: 352,
                        as_str(): "fn main() -> bool {\n    let foo = Foo {\n        foo: \"foo\",\n    };\n    let db = ::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    };\n    let bar = Bar {\n        a: 5u32,\n    };\n    false\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb118764f50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
                        ),
                        start: 170,
                        end: 174,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb118764f50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
            ),
            start: 157,
            end: 352,
            as_str(): "fn main() -> bool {\n    let foo = Foo {\n        foo: \"foo\",\n    };\n    let db = ::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    };\n    let bar = Bar {\n        a: 5u32,\n    };\n    false\n}",
        },
    },
]
