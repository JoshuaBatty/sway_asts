




TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118a4cac0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/a_dependency.sw",
                                            ),
                                            start: 48,
                                            end: 51,
                                            as_str(): "Foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
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
                                            value: TyExpression {
                                                expression: Literal(
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
                                                return_type: TypeId(
                                                    7269,
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
                                        },
                                    ],
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
                                return_type: TypeId(
                                    7263,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7263,
                            ),
                            type_ascription: TypeId(
                                7266,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1084c27b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/double_inner/double_bar.sw",
                                            ),
                                            start: 50,
                                            end: 59,
                                            as_str(): "DoubleBar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
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
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
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
                                        },
                                    ],
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
                                return_type: TypeId(
                                    7259,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7259,
                            ),
                            type_ascription: TypeId(
                                7270,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb118822bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/inner/bar.sw",
                                            ),
                                            start: 82,
                                            end: 85,
                                            as_str(): "Bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
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
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
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
                                        },
                                    ],
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
                                return_type: TypeId(
                                    7264,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7264,
                            ),
                            type_ascription: TypeId(
                                7273,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            Boolean(
                                false,
                            ),
                        ),
                        return_type: TypeId(
                            71,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb118764f50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRm3yWYE/dependencies/src/main.sw",
        ),
        start: 157,
        end: 352,
        as_str(): "fn main() -> bool {\n    let foo = Foo {\n        foo: \"foo\",\n    };\n    let db = ::foo::bar::double_bar::DoubleBar {\n        a: 5u32,\n    };\n    let bar = Bar {\n        a: 5u32,\n    };\n    false\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

