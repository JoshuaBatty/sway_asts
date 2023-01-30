[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a1239c1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
            src (ptr): 0x00007f8a1239c1f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                            src (ptr): 0x00007f8a1239c1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 79,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                ),
                                                                                                start: 104,
                                                                                                end: 105,
                                                                                                as_str(): "!",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                ),
                                                                                                start: 104,
                                                                                                end: 105,
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
                                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                            ),
                                                                                            start: 104,
                                                                                            end: 105,
                                                                                            as_str(): "!",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                ),
                                                                                start: 104,
                                                                                end: 105,
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
                                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                            ),
                                                                                            start: 105,
                                                                                            end: 106,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 105,
                                                                                    end: 106,
                                                                                    as_str(): "b",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 106,
                                                                    as_str(): "!b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 106,
                                                    as_str(): "! !b",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 107,
                                    as_str(): "let c = ! !b;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 125,
                                                    end: 126,
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                ),
                                                                                start: 129,
                                                                                end: 130,
                                                                                as_str(): "!",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                ),
                                                                                start: 129,
                                                                                end: 130,
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
                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                            ),
                                                                            start: 129,
                                                                            end: 130,
                                                                            as_str(): "!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 130,
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
                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 133,
                                                                                                                end: 134,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 133,
                                                                                                                end: 134,
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
                                                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 133,
                                                                                                            end: 134,
                                                                                                            as_str(): "!",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                ),
                                                                                                start: 133,
                                                                                                end: 134,
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
                                                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 134,
                                                                                                            end: 135,
                                                                                                            as_str(): "c",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 134,
                                                                                                    end: 135,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 133,
                                                                                    end: 135,
                                                                                    as_str(): "!c",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 131,
                                                                    end: 135,
                                                                    as_str(): "! !c",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 135,
                                                    as_str(): "! ! !c",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 121,
                                    end: 136,
                                    as_str(): "let d = ! ! !c;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                    ),
                                                    start: 149,
                                                    end: 150,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a1239c1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                            ),
                                            start: 149,
                                            end: 150,
                                            as_str(): "d",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1239c1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                    ),
                                    start: 149,
                                    end: 150,
                                    as_str(): "d",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a1239c1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                            ),
                            start: 39,
                            end: 152,
                            as_str(): "{\n    let a: bool = true;\n    let b = !a; // false\n    let c = ! !b; // false\n    let d = ! ! !c; // true\n    d\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a1239c1f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                        ),
                        start: 21,
                        end: 152,
                        as_str(): "fn main() -> bool {\n    let a: bool = true;\n    let b = !a; // false\n    let c = ! !b; // false\n    let d = ! ! !c; // true\n    d\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a1239c1f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                        ),
                        start: 34,
                        end: 38,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a1239c1f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
            ),
            start: 21,
            end: 152,
            as_str(): "fn main() -> bool {\n    let a: bool = true;\n    let b = !a; // false\n    let c = ! !b; // false\n    let d = ! ! !c; // true\n    d\n}",
        },
    },
]
