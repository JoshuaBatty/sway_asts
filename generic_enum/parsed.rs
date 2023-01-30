[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 18,
                            end: 21,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 23,
                            end: 26,
                            as_str(): "Ord",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0fc4e9d60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
            ),
            start: 8,
            end: 27,
            as_str(): "use core::ops::Ord;",
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
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 32,
                            end: 36,
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
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 57,
                                                    end: 58,
                                                    as_str(): "x",
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
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 67,
                                                                                as_str(): "Option",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 61,
                                                                            end: 67,
                                                                            as_str(): "Option",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 69,
                                                                            end: 73,
                                                                            as_str(): "Some",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 73,
                                                                as_str(): "Option::Some",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                    ),
                                                                    start: 74,
                                                                    end: 76,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 77,
                                                    as_str(): "Option::Some(10)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 53,
                                    end: 78,
                                    as_str(): "let x = Option::Some(10);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 88,
                                                    end: 89,
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
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                ),
                                                                                start: 92,
                                                                                end: 98,
                                                                                as_str(): "Option",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 92,
                                                                            end: 98,
                                                                            as_str(): "Option",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 104,
                                                                            as_str(): "Some",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 104,
                                                                as_str(): "Option::Some",
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
                                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 109,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 110,
                                                    as_str(): "Option::Some(true)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 84,
                                    end: 111,
                                    as_str(): "let y = Option::Some(true);",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 149,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 145,
                                    end: 149,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 47,
                            end: 151,
                            as_str(): "{\n    let x = Option::Some(10); \n    let y = Option::Some(true); \n\n //   x == Option::Some(10)\n   true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fc4e9d60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                        ),
                        start: 29,
                        end: 151,
                        as_str(): "fn main() -> bool {\n    let x = Option::Some(10); \n    let y = Option::Some(true); \n\n //   x == Option::Some(10)\n   true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc4e9d60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                        ),
                        start: 42,
                        end: 46,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc4e9d60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
            ),
            start: 29,
            end: 151,
            as_str(): "fn main() -> bool {\n    let x = Option::Some(10); \n    let y = Option::Some(true); \n\n //   x == Option::Some(10)\n   true\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 158,
                            end: 164,
                            as_str(): "Option",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7249),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 174,
                                    end: 178,
                                    as_str(): "Some",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 180,
                                        end: 181,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc4e9d60,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                ),
                                start: 180,
                                end: 181,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fc4e9d60,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                ),
                                start: 174,
                                end: 181,
                                as_str(): "Some: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc4e9d60,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                    ),
                                    start: 187,
                                    end: 191,
                                    as_str(): "None",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc4e9d60,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                ),
                                start: 193,
                                end: 195,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0fc4e9d60,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                ),
                                start: 187,
                                end: 195,
                                as_str(): "None: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc4e9d60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                        ),
                        start: 153,
                        end: 197,
                        as_str(): "enum Option<T> {\n    Some: T,\n    None: ()\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc4e9d60,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
            ),
            start: 153,
            end: 197,
            as_str(): "enum Option<T> {\n    Some: T,\n    None: ()\n}",
        },
    },
]
