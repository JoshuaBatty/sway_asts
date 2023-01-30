[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 9,
                    end: 19,
                    as_str(): "dep utils;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 13,
                    end: 18,
                    as_str(): "utils",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 9,
            end: 19,
            as_str(): "dep utils;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 25,
                            end: 28,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 30,
                            end: 36,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 38,
                            end: 44,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 21,
            end: 45,
            as_str(): "use std::assert::assert;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 51,
                            end: 56,
                            as_str(): "utils",
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
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 47,
            end: 60,
            as_str(): "use utils::*;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 69,
                            end: 79,
                            as_str(): "CustomType",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 90,
                                    as_str(): "name",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Str(
                                Length {
                                    val: 3,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 96,
                                        end: 97,
                                        as_str(): "3",
                                    },
                                },
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 86,
                                end: 98,
                                as_str(): "name: str[3]",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 92,
                                end: 98,
                                as_str(): "str[3]",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 62,
                        end: 101,
                        as_str(): "struct CustomType {\n    name: str[3],\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 62,
            end: 101,
            as_str(): "struct CustomType {\n    name: str[3],\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 108,
                            end: 116,
                            as_str(): "MyResult",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(31630),
                        E: TypeId(31631),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 131,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 133,
                                        end: 134,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 133,
                                end: 134,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 129,
                                end: 134,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 140,
                                    end: 143,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 145,
                                        end: 146,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 145,
                                end: 146,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 140,
                                end: 146,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 103,
                        end: 149,
                        as_str(): "enum MyResult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 103,
            end: 149,
            as_str(): "enum MyResult<T, E> {\n    Ok: T,\n    Err: E,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 158,
                            end: 168,
                            as_str(): "SomeStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 178,
                                    end: 179,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 181,
                                        end: 182,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 178,
                                end: 182,
                                as_str(): "a: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 181,
                                end: 182,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(31632),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 151,
                        end: 185,
                        as_str(): "struct SomeStruct<T> {\n    a: T,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 151,
            end: 185,
            as_str(): "struct SomeStruct<T> {\n    a: T,\n}",
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
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 190,
                            end: 205,
                            as_str(): "simple_vec_test",
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
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 222,
                                                    end: 226,
                                                    as_str(): "vec1",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 229,
                                                                                end: 232,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 229,
                                                                            end: 232,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 234,
                                                                            end: 237,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 229,
                                                                end: 237,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 239,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 214,
                                    end: 240,
                                    as_str(): "let mut vec1 = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 253,
                                                    end: 257,
                                                    as_str(): "vec2",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 260,
                                                                                end: 263,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 260,
                                                                            end: 263,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 268,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 260,
                                                                end: 268,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 270,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 245,
                                    end: 271,
                                    as_str(): "let mut vec2 = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 282,
                                                                end: 286,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 282,
                                                        end: 286,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 277,
                                                                    end: 281,
                                                                    as_str(): "vec2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 277,
                                                            end: 281,
                                                            as_str(): "vec2",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            U32(
                                                                54,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 287,
                                                            end: 292,
                                                            as_str(): "54u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 277,
                                            end: 293,
                                            as_str(): "vec2.push(54u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 277,
                                    end: 293,
                                    as_str(): "vec2.push(54u32)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 304,
                                                                end: 308,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 304,
                                                        end: 308,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 299,
                                                                    end: 303,
                                                                    as_str(): "vec1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 303,
                                                            as_str(): "vec1",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 309,
                                                                                end: 319,
                                                                                as_str(): "SomeStruct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 309,
                                                                        end: 319,
                                                                        as_str(): "SomeStruct",
                                                                    },
                                                                },
                                                                fields: [
                                                                    StructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 322,
                                                                                end: 323,
                                                                                as_str(): "a",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 325,
                                                                                end: 327,
                                                                                as_str(): "42",
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 322,
                                                                            end: 327,
                                                                            as_str(): "a: 42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 329,
                                                            as_str(): "SomeStruct { a: 42 }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 299,
                                            end: 330,
                                            as_str(): "vec1.push(SomeStruct { a: 42 })",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 299,
                                    end: 330,
                                    as_str(): "vec1.push(SomeStruct { a: 42 })",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 337,
                                                                end: 343,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 337,
                                                        end: 343,
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 367,
                                                                                        end: 369,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 367,
                                                                                        end: 369,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 367,
                                                                                    end: 369,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 367,
                                                                        end: 369,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: MethodApplication(
                                                                                        MethodApplicationExpression {
                                                                                            method_name_binding: TypeBinding {
                                                                                                inner: FromModule {
                                                                                                    method_name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 356,
                                                                                                            end: 362,
                                                                                                            as_str(): "unwrap",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 356,
                                                                                                    end: 362,
                                                                                                    as_str(): "unwrap",
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
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 349,
                                                                                                                            end: 352,
                                                                                                                            as_str(): "get",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 349,
                                                                                                                    end: 352,
                                                                                                                    as_str(): "get",
                                                                                                                },
                                                                                                            },
                                                                                                            contract_call_params: [],
                                                                                                            arguments: [
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 344,
                                                                                                                                end: 348,
                                                                                                                                as_str(): "vec1",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 344,
                                                                                                                        end: 348,
                                                                                                                        as_str(): "vec1",
                                                                                                                    },
                                                                                                                },
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            0,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 353,
                                                                                                                        end: 354,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 344,
                                                                                                        end: 355,
                                                                                                        as_str(): "vec1.get(0)",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 344,
                                                                                        end: 364,
                                                                                        as_str(): "vec1.get(0).unwrap()",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 365,
                                                                                        end: 366,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 366,
                                                                            as_str(): "vec1.get(0).unwrap().a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 370,
                                                                            end: 372,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 372,
                                                            as_str(): "vec1.get(0).unwrap().a == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 337,
                                            end: 373,
                                            as_str(): "assert(vec1.get(0).unwrap().a == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 337,
                                    end: 373,
                                    as_str(): "assert(vec1.get(0).unwrap().a == 42)",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 379,
                                                                end: 385,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 379,
                                                        end: 385,
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 407,
                                                                                        end: 409,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 407,
                                                                                        end: 409,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 407,
                                                                                    end: 409,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 407,
                                                                        end: 409,
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
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 398,
                                                                                                end: 404,
                                                                                                as_str(): "unwrap",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 398,
                                                                                        end: 404,
                                                                                        as_str(): "unwrap",
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
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 391,
                                                                                                                end: 394,
                                                                                                                as_str(): "get",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 391,
                                                                                                        end: 394,
                                                                                                        as_str(): "get",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 386,
                                                                                                                    end: 390,
                                                                                                                    as_str(): "vec2",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 386,
                                                                                                            end: 390,
                                                                                                            as_str(): "vec2",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                0,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 395,
                                                                                                            end: 396,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 386,
                                                                                            end: 397,
                                                                                            as_str(): "vec2.get(0)",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 386,
                                                                            end: 406,
                                                                            as_str(): "vec2.get(0).unwrap()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                54,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 410,
                                                                            end: 415,
                                                                            as_str(): "54u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 386,
                                                            end: 415,
                                                            as_str(): "vec2.get(0).unwrap() == 54u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 379,
                                            end: 416,
                                            as_str(): "assert(vec2.get(0).unwrap() == 54u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 379,
                                    end: 416,
                                    as_str(): "assert(vec2.get(0).unwrap() == 54u32)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 208,
                            end: 419,
                            as_str(): "{\n    let mut vec1 = Vec::new();\n    let mut vec2 = Vec::new();\n\n    vec2.push(54u32);\n    vec1.push(SomeStruct { a: 42 });\n\n    assert(vec1.get(0).unwrap().a == 42);\n    assert(vec2.get(0).unwrap() == 54u32);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 187,
                        end: 419,
                        as_str(): "fn simple_vec_test() {\n    let mut vec1 = Vec::new();\n    let mut vec2 = Vec::new();\n\n    vec2.push(54u32);\n    vec1.push(SomeStruct { a: 42 });\n\n    assert(vec1.get(0).unwrap().a == 42);\n    assert(vec2.get(0).unwrap() == 54u32);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 187,
                        end: 207,
                        as_str(): "fn simple_vec_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 187,
            end: 419,
            as_str(): "fn simple_vec_test() {\n    let mut vec1 = Vec::new();\n    let mut vec2 = Vec::new();\n\n    vec2.push(54u32);\n    vec1.push(SomeStruct { a: 42 });\n\n    assert(vec1.get(0).unwrap().a == 42);\n    assert(vec2.get(0).unwrap() == 54u32);\n}",
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
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 424,
                            end: 440,
                            as_str(): "complex_vec_test",
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
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 494,
                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 497,
                                                                                end: 500,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 497,
                                                                            end: 500,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 502,
                                                                            end: 505,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 497,
                                                                end: 505,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 497,
                                                    end: 507,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 449,
                                    end: 508,
                                    as_str(): "let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 521,
                                                    end: 532,
                                                    as_str(): "inner_vec_1",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 535,
                                                                                end: 538,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 535,
                                                                            end: 538,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 540,
                                                                            end: 543,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 535,
                                                                end: 543,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 535,
                                                    end: 545,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 513,
                                    end: 546,
                                    as_str(): "let mut inner_vec_1 = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 555,
                                                    end: 572,
                                                    as_str(): "inner_inner_vec_1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 575,
                                                                        end: 583,
                                                                        as_str(): "vec_from",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 575,
                                                                end: 583,
                                                                as_str(): "vec_from",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Array(
                                                                    ArrayExpression {
                                                                        contents: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 585,
                                                                                    end: 586,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 588,
                                                                                    end: 589,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        2,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 591,
                                                                                    end: 592,
                                                                                    as_str(): "2",
                                                                                },
                                                                            },
                                                                        ],
                                                                        length_span: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 584,
                                                                    end: 593,
                                                                    as_str(): "[0, 1, 2]",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 575,
                                                    end: 594,
                                                    as_str(): "vec_from([0, 1, 2])",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 551,
                                    end: 595,
                                    as_str(): "let inner_inner_vec_1 = vec_from([0, 1, 2]);",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 613,
                                                                end: 617,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 613,
                                                        end: 617,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 601,
                                                                    end: 612,
                                                                    as_str(): "inner_vec_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 601,
                                                            end: 612,
                                                            as_str(): "inner_vec_1",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 618,
                                                                    end: 635,
                                                                    as_str(): "inner_inner_vec_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 618,
                                                            end: 635,
                                                            as_str(): "inner_inner_vec_1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 601,
                                            end: 636,
                                            as_str(): "inner_vec_1.push(inner_inner_vec_1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 601,
                                    end: 636,
                                    as_str(): "inner_vec_1.push(inner_inner_vec_1)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 680,
                                                                end: 684,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 680,
                                                        end: 684,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 642,
                                                                    end: 679,
                                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 642,
                                                            end: 679,
                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 685,
                                                                                end: 695,
                                                                                as_str(): "SomeStruct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 685,
                                                                        end: 695,
                                                                        as_str(): "SomeStruct",
                                                                    },
                                                                },
                                                                fields: [
                                                                    StructExpressionField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 698,
                                                                                end: 699,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 701,
                                                                                        end: 712,
                                                                                        as_str(): "inner_vec_1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 701,
                                                                                end: 712,
                                                                                as_str(): "inner_vec_1",
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 698,
                                                                            end: 712,
                                                                            as_str(): "a: inner_vec_1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 685,
                                                            end: 714,
                                                            as_str(): "SomeStruct { a: inner_vec_1 }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 642,
                                            end: 715,
                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 })",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 642,
                                    end: 715,
                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 })",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 722,
                                                                end: 728,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 722,
                                                        end: 728,
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 773,
                                                                                        end: 775,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 773,
                                                                                        end: 775,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 773,
                                                                                    end: 775,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 773,
                                                                        end: 775,
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
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 764,
                                                                                                end: 770,
                                                                                                as_str(): "unwrap",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 764,
                                                                                        end: 770,
                                                                                        as_str(): "unwrap",
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
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 757,
                                                                                                                end: 760,
                                                                                                                as_str(): "get",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 757,
                                                                                                        end: 760,
                                                                                                        as_str(): "get",
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
                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 748,
                                                                                                                                end: 754,
                                                                                                                                as_str(): "unwrap",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 748,
                                                                                                                        end: 754,
                                                                                                                        as_str(): "unwrap",
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
                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 741,
                                                                                                                                                end: 744,
                                                                                                                                                as_str(): "get",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 741,
                                                                                                                                        end: 744,
                                                                                                                                        as_str(): "get",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 729,
                                                                                                                                                    end: 740,
                                                                                                                                                    as_str(): "inner_vec_1",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 729,
                                                                                                                                            end: 740,
                                                                                                                                            as_str(): "inner_vec_1",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Literal(
                                                                                                                                            Numeric(
                                                                                                                                                0,
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 745,
                                                                                                                                            end: 746,
                                                                                                                                            as_str(): "0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 729,
                                                                                                                            end: 747,
                                                                                                                            as_str(): "inner_vec_1.get(0)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 729,
                                                                                                            end: 756,
                                                                                                            as_str(): "inner_vec_1.get(0).unwrap()",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                1,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 761,
                                                                                                            end: 762,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 729,
                                                                                            end: 763,
                                                                                            as_str(): "inner_vec_1.get(0).unwrap().get(1)",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 729,
                                                                            end: 772,
                                                                            as_str(): "inner_vec_1.get(0).unwrap().get(1).unwrap()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 776,
                                                                            end: 777,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 729,
                                                            end: 777,
                                                            as_str(): "inner_vec_1.get(0).unwrap().get(1).unwrap() == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 722,
                                            end: 778,
                                            as_str(): "assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 722,
                                    end: 778,
                                    as_str(): "assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1)",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 784,
                                                                end: 790,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 784,
                                                        end: 790,
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
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 879,
                                                                                        end: 881,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 879,
                                                                                        end: 881,
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
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 879,
                                                                                    end: 881,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 879,
                                                                        end: 881,
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
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 870,
                                                                                                end: 876,
                                                                                                as_str(): "unwrap",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 870,
                                                                                        end: 876,
                                                                                        as_str(): "unwrap",
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
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 863,
                                                                                                                end: 866,
                                                                                                                as_str(): "get",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 863,
                                                                                                        end: 866,
                                                                                                        as_str(): "get",
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
                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 854,
                                                                                                                                end: 860,
                                                                                                                                as_str(): "unwrap",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 854,
                                                                                                                        end: 860,
                                                                                                                        as_str(): "unwrap",
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
                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 847,
                                                                                                                                                end: 850,
                                                                                                                                                as_str(): "get",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 847,
                                                                                                                                        end: 850,
                                                                                                                                        as_str(): "get",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Subfield(
                                                                                                                                            SubfieldExpression {
                                                                                                                                                prefix: Expression {
                                                                                                                                                    kind: MethodApplication(
                                                                                                                                                        MethodApplicationExpression {
                                                                                                                                                            method_name_binding: TypeBinding {
                                                                                                                                                                inner: FromModule {
                                                                                                                                                                    method_name: BaseIdent {
                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 836,
                                                                                                                                                                            end: 842,
                                                                                                                                                                            as_str(): "unwrap",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 836,
                                                                                                                                                                    end: 842,
                                                                                                                                                                    as_str(): "unwrap",
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
                                                                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 829,
                                                                                                                                                                                            end: 832,
                                                                                                                                                                                            as_str(): "get",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                                type_arguments: [],
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 829,
                                                                                                                                                                                    end: 832,
                                                                                                                                                                                    as_str(): "get",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            contract_call_params: [],
                                                                                                                                                                            arguments: [
                                                                                                                                                                                Expression {
                                                                                                                                                                                    kind: Variable(
                                                                                                                                                                                        BaseIdent {
                                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                                            span: Span {
                                                                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                                                ),
                                                                                                                                                                                                start: 791,
                                                                                                                                                                                                end: 828,
                                                                                                                                                                                                as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                                                                                                                                                            },
                                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                                        },
                                                                                                                                                                                    ),
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 791,
                                                                                                                                                                                        end: 828,
                                                                                                                                                                                        as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                                Expression {
                                                                                                                                                                                    kind: Literal(
                                                                                                                                                                                        Numeric(
                                                                                                                                                                                            0,
                                                                                                                                                                                        ),
                                                                                                                                                                                    ),
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 833,
                                                                                                                                                                                        end: 834,
                                                                                                                                                                                        as_str(): "0",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                            ],
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 791,
                                                                                                                                                                        end: 835,
                                                                                                                                                                        as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0)",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 791,
                                                                                                                                                        end: 844,
                                                                                                                                                        as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap()",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                field_to_access: BaseIdent {
                                                                                                                                                    name_override_opt: None,
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 845,
                                                                                                                                                        end: 846,
                                                                                                                                                        as_str(): "a",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 791,
                                                                                                                                            end: 846,
                                                                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Literal(
                                                                                                                                            Numeric(
                                                                                                                                                0,
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 851,
                                                                                                                                            end: 852,
                                                                                                                                            as_str(): "0",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 791,
                                                                                                                            end: 853,
                                                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 791,
                                                                                                            end: 862,
                                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap()",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                2,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 867,
                                                                                                            end: 868,
                                                                                                            as_str(): "2",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 791,
                                                                                            end: 869,
                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2)",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 791,
                                                                            end: 878,
                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 882,
                                                                            end: 883,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 791,
                                                            end: 883,
                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 784,
                                            end: 884,
                                            as_str(): "assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 784,
                                    end: 884,
                                    as_str(): "assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 443,
                            end: 887,
                            as_str(): "{\n    let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();\n    let mut inner_vec_1 = Vec::new();\n    let inner_inner_vec_1 = vec_from([0, 1, 2]);\n\n    inner_vec_1.push(inner_inner_vec_1);\n    exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 });\n\n    assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1);\n    assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 421,
                        end: 887,
                        as_str(): "fn complex_vec_test() {\n    let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();\n    let mut inner_vec_1 = Vec::new();\n    let inner_inner_vec_1 = vec_from([0, 1, 2]);\n\n    inner_vec_1.push(inner_inner_vec_1);\n    exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 });\n\n    assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1);\n    assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 421,
                        end: 442,
                        as_str(): "fn complex_vec_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 421,
            end: 887,
            as_str(): "fn complex_vec_test() {\n    let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();\n    let mut inner_vec_1 = Vec::new();\n    let inner_inner_vec_1 = vec_from([0, 1, 2]);\n\n    inner_vec_1.push(inner_inner_vec_1);\n    exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 });\n\n    assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1);\n    assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2);\n}",
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
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 892,
                            end: 919,
                            as_str(): "simple_option_generics_test",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 928,
                                                                end: 934,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 928,
                                                        end: 934,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromModule {
                                                                        method_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 958,
                                                                                end: 965,
                                                                                as_str(): "is_none",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 958,
                                                                        end: 965,
                                                                        as_str(): "is_none",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 935,
                                                                                                end: 948,
                                                                                                as_str(): "get_an_option",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [
                                                                                        TypeArgument {
                                                                                            type_id: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 951,
                                                                                                end: 954,
                                                                                                as_str(): "u64",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 935,
                                                                                        end: 955,
                                                                                        as_str(): "get_an_option::<u64>",
                                                                                    },
                                                                                },
                                                                                arguments: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 935,
                                                                            end: 957,
                                                                            as_str(): "get_an_option::<u64>()",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 935,
                                                            end: 967,
                                                            as_str(): "get_an_option::<u64>().is_none()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 928,
                                            end: 968,
                                            as_str(): "assert(get_an_option::<u64>().is_none())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 928,
                                    end: 968,
                                    as_str(): "assert(get_an_option::<u64>().is_none())",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 922,
                            end: 971,
                            as_str(): "{\n    assert(get_an_option::<u64>().is_none());\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 889,
                        end: 971,
                        as_str(): "fn simple_option_generics_test() {\n    assert(get_an_option::<u64>().is_none());\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 889,
                        end: 921,
                        as_str(): "fn simple_option_generics_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 889,
            end: 971,
            as_str(): "fn simple_option_generics_test() {\n    assert(get_an_option::<u64>().is_none());\n}",
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
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 976,
                            end: 980,
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 989,
                                                                end: 1001,
                                                                as_str(): "sell_product",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 989,
                                                        end: 1001,
                                                        as_str(): "sell_product",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 989,
                                            end: 1003,
                                            as_str(): "sell_product()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 989,
                                    end: 1003,
                                    as_str(): "sell_product()",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1009,
                                                                end: 1024,
                                                                as_str(): "simple_vec_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 1009,
                                                        end: 1024,
                                                        as_str(): "simple_vec_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1009,
                                            end: 1026,
                                            as_str(): "simple_vec_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1009,
                                    end: 1026,
                                    as_str(): "simple_vec_test()",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1032,
                                                                end: 1048,
                                                                as_str(): "complex_vec_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 1032,
                                                        end: 1048,
                                                        as_str(): "complex_vec_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1032,
                                            end: 1050,
                                            as_str(): "complex_vec_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1032,
                                    end: 1050,
                                    as_str(): "complex_vec_test()",
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
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1056,
                                                                end: 1083,
                                                                as_str(): "simple_option_generics_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 1056,
                                                        end: 1083,
                                                        as_str(): "simple_option_generics_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1056,
                                            end: 1085,
                                            as_str(): "simple_option_generics_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1056,
                                    end: 1085,
                                    as_str(): "simple_option_generics_test()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 983,
                            end: 1088,
                            as_str(): "{\n    sell_product();\n    simple_vec_test();\n    complex_vec_test();\n    simple_option_generics_test();\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 973,
                        end: 1088,
                        as_str(): "fn main() {\n    sell_product();\n    simple_vec_test();\n    complex_vec_test();\n    simple_option_generics_test();\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 973,
                        end: 982,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 973,
            end: 1088,
            as_str(): "fn main() {\n    sell_product();\n    simple_vec_test();\n    complex_vec_test();\n    simple_option_generics_test();\n}",
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
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1093,
                            end: 1105,
                            as_str(): "sell_product",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            false,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 1147,
                                                        end: 1152,
                                                        as_str(): "false",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Return(
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
                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1170,
                                                                                                                    end: 1178,
                                                                                                                    as_str(): "MyResult",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1170,
                                                                                                                end: 1178,
                                                                                                                as_str(): "MyResult",
                                                                                                            },
                                                                                                        },
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1180,
                                                                                                                end: 1183,
                                                                                                                as_str(): "Err",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1170,
                                                                                                    end: 1183,
                                                                                                    as_str(): "MyResult::Err",
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
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1184,
                                                                                                                            end: 1194,
                                                                                                                            as_str(): "CustomType",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    is_absolute: false,
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1184,
                                                                                                                    end: 1194,
                                                                                                                    as_str(): "CustomType",
                                                                                                                },
                                                                                                            },
                                                                                                            fields: [
                                                                                                                StructExpressionField {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1209,
                                                                                                                            end: 1213,
                                                                                                                            as_str(): "name",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value: Expression {
                                                                                                                        kind: Literal(
                                                                                                                            String(
                                                                                                                                Span {
                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1216,
                                                                                                                                    end: 1219,
                                                                                                                                    as_str(): "foo",
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1215,
                                                                                                                            end: 1220,
                                                                                                                            as_str(): "\"foo\"",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1209,
                                                                                                                        end: 1220,
                                                                                                                        as_str(): "name: \"foo\"",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1184,
                                                                                                        end: 1230,
                                                                                                        as_str(): "CustomType {\n            name: \"foo\"\n        }",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 1170,
                                                                                        end: 1231,
                                                                                        as_str(): "MyResult::Err(CustomType {\n            name: \"foo\"\n        })",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1163,
                                                                                end: 1231,
                                                                                as_str(): "return MyResult::Err(CustomType {\n            name: \"foo\"\n        })",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1163,
                                                                        end: 1231,
                                                                        as_str(): "return MyResult::Err(CustomType {\n            name: \"foo\"\n        })",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1153,
                                                                end: 1238,
                                                                as_str(): "{\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 1153,
                                                        end: 1238,
                                                        as_str(): "{\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                                                    },
                                                },
                                                else: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1144,
                                            end: 1238,
                                            as_str(): "if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1144,
                                    end: 1238,
                                    as_str(): "if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
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
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1252,
                                                                                end: 1260,
                                                                                as_str(): "MyResult",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1252,
                                                                            end: 1260,
                                                                            as_str(): "MyResult",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1262,
                                                                            end: 1264,
                                                                            as_str(): "Ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 1252,
                                                                end: 1264,
                                                                as_str(): "MyResult::Ok",
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
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1265,
                                                                    end: 1270,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 1252,
                                                    end: 1271,
                                                    as_str(): "MyResult::Ok(false)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1245,
                                            end: 1271,
                                            as_str(): "return MyResult::Ok(false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1245,
                                    end: 1271,
                                    as_str(): "return MyResult::Ok(false)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1138,
                            end: 1274,
                            as_str(): "{\n    if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    };\n\n    return MyResult::Ok(false);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 1090,
                        end: 1274,
                        as_str(): "fn sell_product() -> MyResult<bool, CustomType> {\n    if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    };\n\n    return MyResult::Ok(false);\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0eb7482d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                ),
                                start: 1111,
                                end: 1119,
                                as_str(): "MyResult",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        71,
                                    ),
                                    initial_type_id: TypeId(
                                        71,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1120,
                                        end: 1124,
                                        as_str(): "bool",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        31633,
                                    ),
                                    initial_type_id: TypeId(
                                        31633,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1126,
                                        end: 1136,
                                        as_str(): "CustomType",
                                    },
                                },
                            ],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0eb7482d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                        ),
                        start: 1111,
                        end: 1137,
                        as_str(): "MyResult<bool, CustomType>",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 1090,
            end: 1274,
            as_str(): "fn sell_product() -> MyResult<bool, CustomType> {\n    if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    };\n\n    return MyResult::Ok(false);\n}",
        },
    },
]
