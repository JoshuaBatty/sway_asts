[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 19,
                            end: 22,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 9,
            end: 26,
            as_str(): "use core::ops::*;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 31,
                            end: 34,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 36,
                            end: 42,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 27,
            end: 46,
            as_str(): "use std::assert::*;",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 53,
                            end: 60,
                            as_str(): "Result2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(31628),
                        E: TypeId(31629),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 75,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 77,
                                        end: 78,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 77,
                                end: 78,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 73,
                                end: 78,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 84,
                                    end: 87,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 89,
                                        end: 90,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 89,
                                end: 90,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 84,
                                end: 90,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 48,
                        end: 93,
                        as_str(): "enum Result2<T, E> {\n    Ok: T,\n    Err: E,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 48,
            end: 93,
            as_str(): "enum Result2<T, E> {\n    Ok: T,\n    Err: E,\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [
                        T: TypeId(31632),
                        E: TypeId(31633),
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 106,
                                end: 113,
                                as_str(): "Result2",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        31630,
                                    ),
                                    initial_type_id: TypeId(
                                        31630,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 114,
                                        end: 115,
                                        as_str(): "T",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        31631,
                                    ),
                                    initial_type_id: TypeId(
                                        31631,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 118,
                                        as_str(): "E",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 106,
                        end: 119,
                        as_str(): "Result2<T, E>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 133,
                                    end: 142,
                                    as_str(): "unwrap_or",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Public,
                            body: CodeBlock {
                                contents: [
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
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 182,
                                                                                    end: 186,
                                                                                    as_str(): "self",
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
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 182,
                                                                                            end: 186,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 182,
                                                                                    end: 186,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 291,
                                                                    as_str(): "match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
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
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 182,
                                                                                                end: 186,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 182,
                                                                                        end: 186,
                                                                                        as_str(): "self",
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
                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                            ),
                                                                                                            start: 201,
                                                                                                            end: 208,
                                                                                                            as_str(): "Result2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 210,
                                                                                                        end: 212,
                                                                                                        as_str(): "Ok",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: Variable {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 213,
                                                                                                        end: 224,
                                                                                                        as_str(): "inner_value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 213,
                                                                                                    end: 224,
                                                                                                    as_str(): "inner_value",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 201,
                                                                                                end: 225,
                                                                                                as_str(): "Result2::Ok(inner_value)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 229,
                                                                                                        end: 240,
                                                                                                        as_str(): "inner_value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 229,
                                                                                                end: 240,
                                                                                                as_str(): "inner_value",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 201,
                                                                                            end: 241,
                                                                                            as_str(): "Result2::Ok(inner_value) => inner_value,",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: EnumScrutinee {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                            ),
                                                                                                            start: 254,
                                                                                                            end: 261,
                                                                                                            as_str(): "Result2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 263,
                                                                                                        end: 266,
                                                                                                        as_str(): "Err",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: CatchAll {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 267,
                                                                                                    end: 268,
                                                                                                    as_str(): "_",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 254,
                                                                                                end: 269,
                                                                                                as_str(): "Result2::Err(_)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 273,
                                                                                                        end: 280,
                                                                                                        as_str(): "default",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 273,
                                                                                                end: 280,
                                                                                                as_str(): "default",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 254,
                                                                                            end: 281,
                                                                                            as_str(): "Result2::Err(_) => default,",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 176,
                                                                            end: 291,
                                                                            as_str(): "match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 291,
                                                                    as_str(): "match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 176,
                                                            end: 291,
                                                            as_str(): "match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 176,
                                                    end: 291,
                                                    as_str(): "match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 176,
                                            end: 291,
                                            as_str(): "match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 297,
                                    as_str(): "{\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 143,
                                            end: 147,
                                            as_str(): "self",
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
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 143,
                                        end: 147,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 149,
                                            end: 156,
                                            as_str(): "default",
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
                                                src (ptr): 0x00007fe0f97a4fa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                ),
                                                start: 158,
                                                end: 159,
                                                as_str(): "T",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 159,
                                        as_str(): "T",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 126,
                                end: 297,
                                as_str(): "pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 164,
                                        end: 165,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 164,
                                end: 165,
                                as_str(): "T",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 95,
                        end: 299,
                        as_str(): "impl<T, E> Result2<T, E> {\n    pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 95,
            end: 299,
            as_str(): "impl<T, E> Result2<T, E> {\n    pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }\n}",
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
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 308,
                            end: 322,
                            as_str(): "test_unwrap_or",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Public,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 368,
                                                                end: 374,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 368,
                                                        end: 374,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 419,
                                                                                        end: 421,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 419,
                                                                                        end: 421,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 419,
                                                                                    end: 421,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 419,
                                                                        end: 421,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: MethodApplication(
                                                                            MethodApplicationExpression {
                                                                                method_name_binding: TypeBinding {
                                                                                    inner: FromModule {
                                                                                        method_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 400,
                                                                                                end: 409,
                                                                                                as_str(): "unwrap_or",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 400,
                                                                                        end: 409,
                                                                                        as_str(): "unwrap_or",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
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
                                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 375,
                                                                                                                        end: 382,
                                                                                                                        as_str(): "Result2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 375,
                                                                                                                    end: 382,
                                                                                                                    as_str(): "Result2",
                                                                                                                },
                                                                                                            },
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 384,
                                                                                                                    end: 386,
                                                                                                                    as_str(): "Ok",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    type_arguments: [
                                                                                                        TypeArgument {
                                                                                                            type_id: TypeId(
                                                                                                                31634,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                31634,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                ),
                                                                                                                start: 389,
                                                                                                                end: 390,
                                                                                                                as_str(): "T",
                                                                                                            },
                                                                                                        },
                                                                                                        TypeArgument {
                                                                                                            type_id: TypeId(
                                                                                                                31635,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                31635,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                ),
                                                                                                                start: 392,
                                                                                                                end: 393,
                                                                                                                as_str(): "T",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 375,
                                                                                                        end: 394,
                                                                                                        as_str(): "Result2::Ok::<T, T>",
                                                                                                    },
                                                                                                },
                                                                                                args: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 395,
                                                                                                                    end: 398,
                                                                                                                    as_str(): "val",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                            ),
                                                                                                            start: 395,
                                                                                                            end: 398,
                                                                                                            as_str(): "val",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 375,
                                                                                            end: 399,
                                                                                            as_str(): "Result2::Ok::<T, T>(val)",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 410,
                                                                                                    end: 417,
                                                                                                    as_str(): "default",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 410,
                                                                                            end: 417,
                                                                                            as_str(): "default",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 375,
                                                                            end: 418,
                                                                            as_str(): "Result2::Ok::<T, T>(val).unwrap_or(default)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 422,
                                                                                    end: 425,
                                                                                    as_str(): "val",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 422,
                                                                            end: 425,
                                                                            as_str(): "val",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 375,
                                                            end: 425,
                                                            as_str(): "Result2::Ok::<T, T>(val).unwrap_or(default) == val",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 368,
                                            end: 426,
                                            as_str(): "assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 368,
                                    end: 426,
                                    as_str(): "assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val)",
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 432,
                                                                end: 438,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 432,
                                                        end: 438,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 484,
                                                                                        end: 486,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 484,
                                                                                        end: 486,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 484,
                                                                                    end: 486,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 484,
                                                                        end: 486,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: MethodApplication(
                                                                            MethodApplicationExpression {
                                                                                method_name_binding: TypeBinding {
                                                                                    inner: FromModule {
                                                                                        method_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 465,
                                                                                                end: 474,
                                                                                                as_str(): "unwrap_or",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 465,
                                                                                        end: 474,
                                                                                        as_str(): "unwrap_or",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
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
                                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 439,
                                                                                                                        end: 446,
                                                                                                                        as_str(): "Result2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 439,
                                                                                                                    end: 446,
                                                                                                                    as_str(): "Result2",
                                                                                                                },
                                                                                                            },
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 448,
                                                                                                                    end: 451,
                                                                                                                    as_str(): "Err",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    type_arguments: [
                                                                                                        TypeArgument {
                                                                                                            type_id: TypeId(
                                                                                                                31636,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                31636,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                ),
                                                                                                                start: 454,
                                                                                                                end: 455,
                                                                                                                as_str(): "T",
                                                                                                            },
                                                                                                        },
                                                                                                        TypeArgument {
                                                                                                            type_id: TypeId(
                                                                                                                31637,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                31637,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                ),
                                                                                                                start: 457,
                                                                                                                end: 458,
                                                                                                                as_str(): "T",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 439,
                                                                                                        end: 459,
                                                                                                        as_str(): "Result2::Err::<T, T>",
                                                                                                    },
                                                                                                },
                                                                                                args: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 460,
                                                                                                                    end: 463,
                                                                                                                    as_str(): "val",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                            ),
                                                                                                            start: 460,
                                                                                                            end: 463,
                                                                                                            as_str(): "val",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 439,
                                                                                            end: 464,
                                                                                            as_str(): "Result2::Err::<T, T>(val)",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 475,
                                                                                                    end: 482,
                                                                                                    as_str(): "default",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 475,
                                                                                            end: 482,
                                                                                            as_str(): "default",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 439,
                                                                            end: 483,
                                                                            as_str(): "Result2::Err::<T, T>(val).unwrap_or(default)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 487,
                                                                                    end: 494,
                                                                                    as_str(): "default",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 487,
                                                                            end: 494,
                                                                            as_str(): "default",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 439,
                                                            end: 494,
                                                            as_str(): "Result2::Err::<T, T>(val).unwrap_or(default) == default",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 432,
                                            end: 495,
                                            as_str(): "assert(Result2::Err::<T, T>(val).unwrap_or(default) == default)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 432,
                                    end: 495,
                                    as_str(): "assert(Result2::Err::<T, T>(val).unwrap_or(default) == default)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 362,
                            end: 498,
                            as_str(): "{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 326,
                                    end: 329,
                                    as_str(): "val",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 331,
                                        end: 332,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 331,
                                end: 332,
                                as_str(): "T",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 334,
                                    end: 341,
                                    as_str(): "default",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 343,
                                        end: 344,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0f97a4fa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                ),
                                start: 343,
                                end: 344,
                                as_str(): "T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 301,
                        end: 498,
                        as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq\n{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [
                        T: TypeId(31638),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 301,
                        end: 361,
                        as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 301,
            end: 498,
            as_str(): "pub fn test_unwrap_or<T>(val: T, default: T)\nwhere\n    T: Eq\n{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
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
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 503,
                            end: 507,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 522,
                                                                end: 536,
                                                                as_str(): "test_unwrap_or",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 522,
                                                        end: 536,
                                                        as_str(): "test_unwrap_or",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 537,
                                                            end: 541,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 543,
                                                            end: 547,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 522,
                                            end: 548,
                                            as_str(): "test_unwrap_or(true, true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 522,
                                    end: 548,
                                    as_str(): "test_unwrap_or(true, true)",
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 552,
                                                                end: 566,
                                                                as_str(): "test_unwrap_or",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 552,
                                                        end: 566,
                                                        as_str(): "test_unwrap_or",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 567,
                                                            end: 571,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Boolean(
                                                                false,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 573,
                                                            end: 578,
                                                            as_str(): "false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 552,
                                            end: 579,
                                            as_str(): "test_unwrap_or(true, false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 552,
                                    end: 579,
                                    as_str(): "test_unwrap_or(true, false)",
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 585,
                                            end: 589,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f97a4fa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                    ),
                                    start: 585,
                                    end: 589,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 518,
                            end: 591,
                            as_str(): "{\n  test_unwrap_or(true, true);\n  test_unwrap_or(true, false);\n\n\n  true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 500,
                        end: 591,
                        as_str(): "fn main() -> bool {\n  test_unwrap_or(true, true);\n  test_unwrap_or(true, false);\n\n\n  true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                        ),
                        start: 513,
                        end: 517,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0f97a4fa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
            ),
            start: 500,
            end: 591,
            as_str(): "fn main() -> bool {\n  test_unwrap_or(true, true);\n  test_unwrap_or(true, false);\n\n\n  true\n}",
        },
    },
]
