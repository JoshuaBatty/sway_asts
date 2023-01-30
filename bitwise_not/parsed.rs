[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 12,
                            end: 16,
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
                                                                src (ptr): 0x00007fb12632e1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 39,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 33,
                                                        end: 39,
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
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 45,
                                                                                        end: 47,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 45,
                                                                                        end: 47,
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
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 45,
                                                                                    end: 47,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 45,
                                                                        end: 47,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                        ),
                                                                                                        start: 40,
                                                                                                        end: 41,
                                                                                                        as_str(): "!",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                        ),
                                                                                                        start: 40,
                                                                                                        end: 41,
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
                                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                    ),
                                                                                                    start: 40,
                                                                                                    end: 41,
                                                                                                    as_str(): "!",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 40,
                                                                                        end: 41,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U8(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 41,
                                                                                            end: 44,
                                                                                            as_str(): "2u8",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 40,
                                                                            end: 44,
                                                                            as_str(): "!2u8",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U8(
                                                                                253,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 48,
                                                                            end: 53,
                                                                            as_str(): "253u8",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 53,
                                                            as_str(): "!2u8 == 253u8",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 33,
                                            end: 54,
                                            as_str(): "assert(!2u8 == 253u8)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12632e1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                    ),
                                    start: 33,
                                    end: 54,
                                    as_str(): "assert(!2u8 == 253u8)",
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
                                                                src (ptr): 0x00007fb12632e1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                ),
                                                                start: 60,
                                                                end: 66,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 60,
                                                        end: 66,
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
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 73,
                                                                                        end: 75,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 73,
                                                                                        end: 75,
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
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 73,
                                                                                    end: 75,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 73,
                                                                        end: 75,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                        ),
                                                                                                        start: 67,
                                                                                                        end: 68,
                                                                                                        as_str(): "!",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                        ),
                                                                                                        start: 67,
                                                                                                        end: 68,
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
                                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                    ),
                                                                                                    start: 67,
                                                                                                    end: 68,
                                                                                                    as_str(): "!",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 67,
                                                                                        end: 68,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U16(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 68,
                                                                                            end: 72,
                                                                                            as_str(): "2u16",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 67,
                                                                            end: 72,
                                                                            as_str(): "!2u16",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U16(
                                                                                65533,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 76,
                                                                            end: 84,
                                                                            as_str(): "65533u16",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 84,
                                                            as_str(): "!2u16 == 65533u16",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 60,
                                            end: 85,
                                            as_str(): "assert(!2u16 == 65533u16)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12632e1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                    ),
                                    start: 60,
                                    end: 85,
                                    as_str(): "assert(!2u16 == 65533u16)",
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
                                                                src (ptr): 0x00007fb12632e1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 97,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 91,
                                                        end: 97,
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
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 104,
                                                                                        end: 106,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 104,
                                                                                        end: 106,
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
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 106,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 104,
                                                                        end: 106,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                        ),
                                                                                                        start: 98,
                                                                                                        end: 99,
                                                                                                        as_str(): "!",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                        ),
                                                                                                        start: 98,
                                                                                                        end: 99,
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
                                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                    ),
                                                                                                    start: 98,
                                                                                                    end: 99,
                                                                                                    as_str(): "!",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 98,
                                                                                        end: 99,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U32(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 99,
                                                                                            end: 103,
                                                                                            as_str(): "2u32",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 103,
                                                                            as_str(): "!2u32",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U32(
                                                                                4294967293,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 107,
                                                                            end: 120,
                                                                            as_str(): "4294967293u32",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 120,
                                                            as_str(): "!2u32 == 4294967293u32",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 91,
                                            end: 121,
                                            as_str(): "assert(!2u32 == 4294967293u32)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12632e1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 121,
                                    as_str(): "assert(!2u32 == 4294967293u32)",
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
                                                                src (ptr): 0x00007fb12632e1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                ),
                                                                start: 127,
                                                                end: 133,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb12632e1f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                        ),
                                                        start: 127,
                                                        end: 133,
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
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 140,
                                                                                        end: 142,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 140,
                                                                                        end: 142,
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
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 140,
                                                                                    end: 142,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 140,
                                                                        end: 142,
                                                                        as_str(): "==",
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
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 134,
                                                                                        end: 135,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U64(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 135,
                                                                                            end: 139,
                                                                                            as_str(): "2u64",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 139,
                                                                            as_str(): "!2u64",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            U64(
                                                                                18446744073709551613,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 143,
                                                                            end: 166,
                                                                            as_str(): "18446744073709551613u64",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 166,
                                                            as_str(): "!2u64 == 18446744073709551613u64",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 127,
                                            end: 167,
                                            as_str(): "assert(!2u64 == 18446744073709551613u64)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12632e1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                    ),
                                    start: 127,
                                    end: 167,
                                    as_str(): "assert(!2u64 == 18446744073709551613u64)",
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
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 174,
                                            end: 178,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12632e1f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                    ),
                                    start: 174,
                                    end: 178,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 27,
                            end: 180,
                            as_str(): "{\n    assert(!2u8 == 253u8);\n    assert(!2u16 == 65533u16);\n    assert(!2u32 == 4294967293u32);\n    assert(!2u64 == 18446744073709551613u64);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12632e1f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                        ),
                        start: 9,
                        end: 180,
                        as_str(): "fn main() -> bool {\n    assert(!2u8 == 253u8);\n    assert(!2u16 == 65533u16);\n    assert(!2u32 == 4294967293u32);\n    assert(!2u64 == 18446744073709551613u64);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12632e1f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                        ),
                        start: 22,
                        end: 26,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12632e1f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
            ),
            start: 9,
            end: 180,
            as_str(): "fn main() -> bool {\n    assert(!2u8 == 253u8);\n    assert(!2u16 == 65533u16);\n    assert(!2u32 == 4294967293u32);\n    assert(!2u64 == 18446744073709551613u64);\n\n    true\n}",
        },
    },
]
