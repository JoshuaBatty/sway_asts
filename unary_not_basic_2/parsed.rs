[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 12,
                            end: 15,
                            as_str(): "std",
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
            src (ptr): 0x00007f8a158f2440,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
            ),
            start: 8,
            end: 19,
            as_str(): "use std::*;",
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
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 24,
                            end: 28,
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 49,
                                                    end: 50,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Boolean,
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 52,
                                                    end: 56,
                                                    as_str(): "bool",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 59,
                                                    end: 63,
                                                    as_str(): "true",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 45,
                                    end: 64,
                                    as_str(): "let a: bool = true;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 73,
                                                    end: 74,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 77,
                                                                                end: 78,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 77,
                                                                                end: 78,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "not",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 77,
                                                                            end: 78,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 78,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 79,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 79,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 79,
                                                    as_str(): "!a",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 80,
                                    as_str(): "let b = !a;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 98,
                                                    end: 99,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 102,
                                                                                end: 103,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 102,
                                                                                end: 103,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "not",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 102,
                                                                            end: 103,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        contract_call_params: [],
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
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 105,
                                                                                                end: 106,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 105,
                                                                                                end: 106,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "not",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 105,
                                                                                            end: 106,
                                                                                            as_str(): "!",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 105,
                                                                                end: 106,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 106,
                                                                                            end: 107,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 106,
                                                                                    end: 107,
                                                                                    as_str(): "b",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 107,
                                                                    as_str(): "!b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 108,
                                                    as_str(): "!( !b)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 109,
                                    as_str(): "let c = !( !b);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 127,
                                                    end: 128,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 131,
                                                                                end: 132,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 131,
                                                                                end: 132,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "not",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 131,
                                                                            end: 132,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 132,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        contract_call_params: [],
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
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 134,
                                                                                                end: 135,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 134,
                                                                                                end: 135,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "not",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 134,
                                                                                            end: 135,
                                                                                            as_str(): "!",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 134,
                                                                                end: 135,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
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
                                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                                ),
                                                                                                                start: 136,
                                                                                                                end: 137,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                                ),
                                                                                                                start: 136,
                                                                                                                end: 137,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "not",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                            ),
                                                                                                            start: 136,
                                                                                                            end: 137,
                                                                                                            as_str(): "!",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 136,
                                                                                                end: 137,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                            ),
                                                                                                            start: 137,
                                                                                                            end: 138,
                                                                                                            as_str(): "c",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                    ),
                                                                                                    start: 137,
                                                                                                    end: 138,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 136,
                                                                                    end: 138,
                                                                                    as_str(): "!c",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 138,
                                                                    as_str(): "! !c",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 131,
                                                    end: 139,
                                                    as_str(): "!( ! !c)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 123,
                                    end: 140,
                                    as_str(): "let d = !( ! !c);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 158,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 163,
                                                                                end: 164,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 163,
                                                                                end: 164,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "not",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 163,
                                                                            end: 164,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 163,
                                                                end: 164,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        contract_call_params: [],
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
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 165,
                                                                                                end: 166,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 165,
                                                                                                end: 166,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "not",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 165,
                                                                                            end: 166,
                                                                                            as_str(): "!",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 165,
                                                                                end: 166,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
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
                                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                                ),
                                                                                                                start: 167,
                                                                                                                end: 168,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                                ),
                                                                                                                start: 167,
                                                                                                                end: 168,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "not",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                            ),
                                                                                                            start: 167,
                                                                                                            end: 168,
                                                                                                            as_str(): "!",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 167,
                                                                                                end: 168,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                            ),
                                                                                                            start: 169,
                                                                                                            end: 170,
                                                                                                            as_str(): "d",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                    ),
                                                                                                    start: 169,
                                                                                                    end: 170,
                                                                                                    as_str(): "d",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 167,
                                                                                    end: 171,
                                                                                    as_str(): "!(d)",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 171,
                                                                    as_str(): "! !(d)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 163,
                                                    end: 171,
                                                    as_str(): "! ! !(d)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 153,
                                    end: 173,
                                    as_str(): "let e = ( ! ! !(d));",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: LazyOperator(
                                            LazyOperatorExpression {
                                                op: Or,
                                                lhs: Expression {
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
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 178,
                                                                                    end: 179,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 178,
                                                                                    end: 179,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "not",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 178,
                                                                                end: 179,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 179,
                                                                    as_str(): "!",
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
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 180,
                                                                                            end: 188,
                                                                                            as_str(): "and_true",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 180,
                                                                                    end: 188,
                                                                                    as_str(): "and_true",
                                                                                },
                                                                            },
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 189,
                                                                                                end: 190,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a158f2440,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                        ),
                                                                                        start: 189,
                                                                                        end: 190,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a158f2440,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                        ),
                                                                        start: 180,
                                                                        end: 191,
                                                                        as_str(): "and_true(a)",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 178,
                                                        end: 192,
                                                        as_str(): "!(and_true(a))",
                                                    },
                                                },
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 196,
                                                        end: 200,
                                                        as_str(): "true",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 178,
                                            end: 200,
                                            as_str(): "!(and_true(a)) || true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 178,
                                    end: 200,
                                    as_str(): "!(and_true(a)) || true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 39,
                            end: 202,
                            as_str(): "{\n    let a: bool = true;\n    let b = !a; // false\n    let c = !( !b); // false\n    let d = !( ! !c); // true\n    let e = ( ! ! !(d));\n    !(and_true(a)) || true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a158f2440,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                        ),
                        start: 21,
                        end: 202,
                        as_str(): "fn main() -> bool {\n    let a: bool = true;\n    let b = !a; // false\n    let c = !( !b); // false\n    let d = !( ! !c); // true\n    let e = ( ! ! !(d));\n    !(and_true(a)) || true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a158f2440,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                        ),
                        start: 34,
                        end: 38,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a158f2440,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
            ),
            start: 21,
            end: 202,
            as_str(): "fn main() -> bool {\n    let a: bool = true;\n    let b = !a; // false\n    let c = !( !b); // false\n    let d = !( ! !c); // true\n    let e = ( ! ! !(d));\n    !(and_true(a)) || true\n}",
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
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 207,
                            end: 215,
                            as_str(): "and_true",
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
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 243,
                                                    end: 244,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 247,
                                                                                end: 248,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 247,
                                                                                end: 248,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "not",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 247,
                                                                            end: 248,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 247,
                                                                end: 248,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        contract_call_params: [],
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
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 249,
                                                                                                end: 250,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 249,
                                                                                                end: 250,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "not",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 249,
                                                                                            end: 250,
                                                                                            as_str(): "!",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 249,
                                                                                end: 250,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        contract_call_params: [],
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 250,
                                                                                            end: 251,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 250,
                                                                                    end: 251,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 249,
                                                                    end: 251,
                                                                    as_str(): "!x",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 247,
                                                    end: 251,
                                                    as_str(): "! !x",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 239,
                                    end: 252,
                                    as_str(): "let y = ! !x;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: LazyOperator(
                                            LazyOperatorExpression {
                                                op: And,
                                                lhs: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 257,
                                                                end: 258,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 257,
                                                        end: 258,
                                                        as_str(): "x",
                                                    },
                                                },
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 262,
                                                        end: 266,
                                                        as_str(): "true",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 257,
                                            end: 266,
                                            as_str(): "x && true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 257,
                                    end: 266,
                                    as_str(): "x && true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 233,
                            end: 268,
                            as_str(): "{\n    let y = ! !x;\n    x && true\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a158f2440,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                    ),
                                    start: 216,
                                    end: 217,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007f8a28022a30,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007f8a158f2440,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                ),
                                start: 219,
                                end: 223,
                                as_str(): "bool",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007f8a158f2440,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                        ),
                        start: 204,
                        end: 268,
                        as_str(): "fn and_true(x: bool) -> bool {\n    let y = ! !x;\n    x && true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a158f2440,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                        ),
                        start: 228,
                        end: 232,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a158f2440,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
            ),
            start: 204,
            end: 268,
            as_str(): "fn and_true(x: bool) -> bool {\n    let y = ! !x;\n    x && true\n}",
        },
    },
]
